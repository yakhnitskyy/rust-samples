//! Desktop window configuration and the root Dioxus component.

use crate::session::AppModel;
use crate::ui::{error_view, question_view, results_view};
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;

const APP_CSS: &str = include_str!("../assets/main.css");

pub(crate) fn launch() {
    let window = WindowBuilder::new()
        .with_title("Rust Knowledge Tester")
        .with_inner_size(LogicalSize::new(1100.0, 900.0))
        .with_min_inner_size(LogicalSize::new(900.0, 720.0))
        .with_resizable(true);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window))
        .launch(app);
}

fn app() -> Element {
    let model = use_signal(AppModel::load);
    let snapshot = model.read().clone();

    rsx! {
        document::Title { "Rust Knowledge Tester" }
        style { dangerous_inner_html: APP_CSS }
        main { class: "app-shell",
            match snapshot.session {
                Some(session) if session.submitted => results_view(session, model),
                Some(session) => question_view(session, model),
                None => error_view(snapshot.error.unwrap_or_else(|| "Unknown error".into()), model),
            }
        }
    }
}
