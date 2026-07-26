//! Mutable state for one quiz attempt and for the application as a whole.

use rust_knowledge_tester::{
    QUIZ_SIZE, Question, load_question_bank, select_questions, validate_question_bank,
};
use std::collections::BTreeSet;

#[derive(Clone)]
pub(crate) struct QuizSession {
    pub(crate) questions: Vec<Question>,
    pub(crate) answers: Vec<BTreeSet<usize>>,
    pub(crate) current: usize,
    pub(crate) submitted: bool,
}

impl QuizSession {
    fn load() -> Result<Self, String> {
        let bank = load_question_bank()?;
        validate_question_bank(&bank)?;
        let questions = select_questions(&bank, QUIZ_SIZE, &mut rand::rng())?;
        Ok(Self {
            answers: vec![BTreeSet::new(); questions.len()],
            questions,
            current: 0,
            submitted: false,
        })
    }

    pub(crate) fn select_answer(&mut self, option: usize) {
        let selected = &mut self.answers[self.current];
        if self.questions[self.current].allows_multiple_answers() {
            if !selected.remove(&option) {
                selected.insert(option);
            }
        } else {
            selected.clear();
            selected.insert(option);
        }
    }
}

#[derive(Clone)]
pub(crate) struct AppModel {
    pub(crate) session: Option<QuizSession>,
    pub(crate) error: Option<String>,
}

impl AppModel {
    pub(crate) fn load() -> Self {
        match QuizSession::load() {
            Ok(session) => Self {
                session: Some(session),
                error: None,
            },
            Err(error) => Self {
                session: None,
                error: Some(error),
            },
        }
    }
}
