//! The active-question screen and its navigation controls.

use crate::session::{AppModel, QuizSession};
use dioxus::prelude::*;
use rust_knowledge_tester::PASS_PERCENT;

pub(crate) fn question_view(session: QuizSession, mut model: Signal<AppModel>) -> Element {
    let question_index = session.current;
    let question = &session.questions[question_index];
    let selected = &session.answers[question_index];
    let is_last = question_index + 1 == session.questions.len();
    let answered = !selected.is_empty();
    let progress = ((question_index + 1) * 100) / session.questions.len();

    rsx! {
        section { class: "quiz-card",
            header { class: "quiz-header",
                div {
                    p { class: "eyebrow", "Rust Knowledge Tester" }
                    h1 { "Question {question_index + 1} of {session.questions.len()}" }
                }
                span { class: "score-rule", "Pass: {PASS_PERCENT}%" }
            }

            div { class: "progress-track",
                div { class: "progress-fill", style: "width: {progress}%;" }
            }

            div { class: "question-body",
                div { class: "question-meta",
                    span { class: "question-number", "#{question.id}" }
                    span { class: "answer-mode",
                        if question.allows_multiple_answers() {
                            "Select all correct answers"
                        } else {
                            "Select one answer"
                        }
                    }
                }
                h2 { "{question.prompt}" }

                div { class: "options",
                    for (option_index, option) in question.options.iter().enumerate() {
                        {
                            let option_class = if selected.contains(&option_index) {
                                "option selected"
                            } else {
                                "option"
                            };
                            let input_type = if question.allows_multiple_answers() {
                                "checkbox"
                            } else {
                                "radio"
                            };
                            rsx! {
                                label {
                                    key: "{question.id}-{option_index}",
                                    class: option_class,
                                    input {
                                        r#type: input_type,
                                        name: "answer",
                                        checked: selected.contains(&option_index),
                                        onchange: move |_| {
                                            if let Some(quiz) = model.write().session.as_mut() {
                                                quiz.select_answer(option_index);
                                            }
                                        },
                                    }
                                    span { class: "option-letter", "{option_letter(option_index)}" }
                                    span { class: "option-text", {option.to_string()} }
                                }
                            }
                        }
                    }
                }
            }

            footer { class: "quiz-footer",
                button {
                    class: "button secondary",
                    disabled: question_index == 0,
                    onclick: move |_| {
                        if let Some(quiz) = model.write().session.as_mut() {
                            quiz.current = quiz.current.saturating_sub(1);
                        }
                    },
                    "Previous"
                }
                span { class: "answered-count",
                    "{session.answers.iter().filter(|answer| !answer.is_empty()).count()} / {session.questions.len()} answered"
                }
                if is_last {
                    button {
                        class: "button primary",
                        disabled: !answered,
                        onclick: move |_| {
                            if let Some(quiz) = model.write().session.as_mut() {
                                quiz.submitted = true;
                            }
                        },
                        "Submit test"
                    }
                } else {
                    button {
                        class: "button primary",
                        disabled: !answered,
                        onclick: move |_| {
                            if let Some(quiz) = model.write().session.as_mut() {
                                quiz.current += 1;
                            }
                        },
                        "Next"
                    }
                }
            }
        }
    }
}

fn option_letter(index: usize) -> char {
    char::from(b'A' + u8::try_from(index).unwrap_or(0))
}
