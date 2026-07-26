#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

mod app;
mod session;
mod ui;

fn main() {
    app::launch();
}
