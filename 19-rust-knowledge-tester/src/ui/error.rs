//! Recoverable question-bank loading error screen.

use crate::session::AppModel;
use dioxus::prelude::*;

pub(crate) fn error_view(error: String, mut model: Signal<AppModel>) -> Element {
    rsx! {
        section { class: "error-card",
            p { class: "eyebrow", "Question bank error" }
            h1 { "The quiz could not start" }
            p { {error} }
            button {
                class: "button primary",
                onclick: move |_| *model.write() = AppModel::load(),
                "Try again"
            }
        }
    }
}
