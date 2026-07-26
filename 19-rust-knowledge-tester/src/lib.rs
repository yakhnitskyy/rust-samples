//! Data loading, random selection, and scoring for the desktop quiz.

use rand::Rng;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::PathBuf;

pub const QUIZ_SIZE: usize = 15;
pub const PASS_PERCENT: usize = 80;

const EMBEDDED_QUESTIONS: &str = include_str!("../assets/questions.json");

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Question {
    pub id: u32,
    pub prompt: String,
    pub options: Vec<String>,
    pub correct_answers: Vec<usize>,
    pub explanation: String,
}

impl Question {
    pub fn allows_multiple_answers(&self) -> bool {
        self.correct_answers.len() > 1
    }
}

/// Reads the editable JSON file during development and falls back to the
/// compile-time copy when a packaged application has no source tree nearby.
pub fn load_question_bank() -> Result<Vec<Question>, String> {
    let source_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("questions.json");
    let json = fs::read_to_string(&source_path).unwrap_or_else(|_| EMBEDDED_QUESTIONS.to_owned());
    serde_json::from_str(&json)
        .map_err(|error| format!("could not parse {}: {error}", source_path.display()))
}

pub fn validate_question_bank(questions: &[Question]) -> Result<(), String> {
    if questions.len() < QUIZ_SIZE {
        return Err(format!(
            "question bank needs at least {QUIZ_SIZE} entries, found {}",
            questions.len()
        ));
    }

    let mut ids = HashSet::new();
    let mut has_single = false;
    let mut has_multiple = false;

    for question in questions {
        if !ids.insert(question.id) {
            return Err(format!("duplicate question id {}", question.id));
        }
        if question.prompt.trim().is_empty() {
            return Err(format!("question {} has an empty prompt", question.id));
        }
        if question.options.len() < 2 {
            return Err(format!(
                "question {} needs at least two options",
                question.id
            ));
        }
        if question.correct_answers.is_empty() {
            return Err(format!("question {} has no correct answer", question.id));
        }

        let unique_answers: HashSet<_> = question.correct_answers.iter().copied().collect();
        if unique_answers.len() != question.correct_answers.len() {
            return Err(format!("question {} repeats a correct answer", question.id));
        }
        if question
            .correct_answers
            .iter()
            .any(|answer| *answer >= question.options.len())
        {
            return Err(format!(
                "question {} has an invalid answer index",
                question.id
            ));
        }

        has_single |= question.correct_answers.len() == 1;
        has_multiple |= question.correct_answers.len() > 1;
    }

    if !has_single || !has_multiple {
        return Err("question bank must contain single and multiple-answer questions".into());
    }
    Ok(())
}

pub fn select_questions<R: Rng + ?Sized>(
    bank: &[Question],
    count: usize,
    rng: &mut R,
) -> Result<Vec<Question>, String> {
    if count > bank.len() {
        return Err(format!(
            "cannot select {count} questions from a bank of {}",
            bank.len()
        ));
    }
    let mut indexes: Vec<usize> = (0..bank.len()).collect();
    indexes.shuffle(rng);
    Ok(indexes
        .into_iter()
        .take(count)
        .map(|index| bank[index].clone())
        .collect())
}

pub fn is_answer_correct(question: &Question, selected: &BTreeSet<usize>) -> bool {
    let expected: BTreeSet<_> = question.correct_answers.iter().copied().collect();
    *selected == expected
}

pub fn score_answers(questions: &[Question], answers: &[BTreeSet<usize>]) -> usize {
    questions
        .iter()
        .zip(answers)
        .filter(|(question, answer)| is_answer_correct(question, answer))
        .count()
}

pub fn percentage(score: usize, total: usize) -> usize {
    score
        .saturating_mul(100)
        .checked_div(total)
        .unwrap_or_default()
}

pub fn passed(score: usize, total: usize) -> bool {
    total > 0 && score * 100 >= PASS_PERCENT * total
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn bank() -> Vec<Question> {
        load_question_bank().expect("the bundled question bank should parse")
    }

    #[test]
    fn bank_contains_70_valid_unique_questions() {
        let questions = bank();
        assert_eq!(questions.len(), 70);
        assert!(validate_question_bank(&questions).is_ok());
    }

    #[test]
    fn bank_contains_single_and_multiple_answer_questions() {
        let questions = bank();
        assert!(
            questions
                .iter()
                .any(|question| !question.allows_multiple_answers())
        );
        assert!(questions.iter().any(Question::allows_multiple_answers));
    }

    #[test]
    fn random_selection_has_15_unique_questions() {
        let questions = bank();
        let mut rng = StdRng::seed_from_u64(42);
        let selected = select_questions(&questions, QUIZ_SIZE, &mut rng).unwrap();
        let ids: HashSet<_> = selected.iter().map(|question| question.id).collect();
        assert_eq!(selected.len(), QUIZ_SIZE);
        assert_eq!(ids.len(), QUIZ_SIZE);
    }

    #[test]
    fn scoring_requires_an_exact_answer_set() {
        let question = Question {
            id: 1,
            prompt: "Select both".into(),
            options: vec!["A".into(), "B".into(), "C".into()],
            correct_answers: vec![0, 2],
            explanation: String::new(),
        };
        assert!(is_answer_correct(&question, &BTreeSet::from([0, 2])));
        assert!(!is_answer_correct(&question, &BTreeSet::from([0])));
        assert!(!is_answer_correct(&question, &BTreeSet::from([0, 1, 2])));
    }

    #[test]
    fn twelve_of_fifteen_is_the_pass_boundary() {
        assert!(!passed(11, 15));
        assert!(passed(12, 15));
        assert_eq!(percentage(12, 15), 80);
    }
}
