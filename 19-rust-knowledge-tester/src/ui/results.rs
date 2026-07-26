//! Final score, pass/fail messaging, and detailed answer review.

use crate::session::{AppModel, QuizSession};
use dioxus::prelude::*;
use rust_knowledge_tester::{Question, is_answer_correct, passed, percentage, score_answers};
use std::collections::BTreeSet;

pub(crate) fn results_view(session: QuizSession, mut model: Signal<AppModel>) -> Element {
    let score = score_answers(&session.questions, &session.answers);
    let percent = percentage(score, session.questions.len());
    let did_pass = passed(score, session.questions.len());

    rsx! {
        section { class: "results-card",
            div { class: if did_pass { "result-banner pass" } else { "result-banner fail" },
                p { class: "eyebrow", "Final result" }
                h1 {
                    if did_pass {
                        "Congratulations! You passed!"
                    } else {
                        "You failed the test."
                    }
                }
                p { class: "result-score",
                    "{score} / {session.questions.len()} correct · {percent}%"
                }
                p {
                    if did_pass {
                        "You reached the required 80% score."
                    } else {
                        "You need at least 12 correct answers out of 15 to pass."
                    }
                }
                button {
                    class: "button restart",
                    onclick: move |_| *model.write() = AppModel::load(),
                    "Try another set"
                }
            }

            div { class: "review",
                h2 { "Answer review" }
                for (index, question) in session.questions.iter().enumerate() {
                    {
                        let correct = is_answer_correct(question, &session.answers[index]);
                        let selected_text = answer_text(question, &session.answers[index]);
                        let expected: BTreeSet<_> =
                            question.correct_answers.iter().copied().collect();
                        let correct_text = answer_text(question, &expected);
                        rsx! {
                            article {
                                key: "{question.id}",
                                class: if correct { "review-item correct" } else { "review-item incorrect" },
                                div { class: "review-heading",
                                    span { class: "review-status",
                                        if correct { "Correct" } else { "Incorrect" }
                                    }
                                    h3 { "{index + 1}. {question.prompt}" }
                                }
                                p { strong { "Your answer: " } {selected_text} }
                                if !correct {
                                    p { strong { "Correct answer: " } {correct_text} }
                                }
                                p { class: "explanation", "{question.explanation}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn answer_text(question: &Question, answers: &BTreeSet<usize>) -> String {
    if answers.is_empty() {
        return "No answer selected".into();
    }
    answers
        .iter()
        .filter_map(|index| question.options.get(*index))
        .cloned()
        .collect::<Vec<_>>()
        .join("; ")
}
