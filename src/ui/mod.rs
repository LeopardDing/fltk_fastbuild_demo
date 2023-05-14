mod config;
mod env_dialog;
mod env_select;
mod mainwindow;
mod message;
mod module_dialog;
mod params;
mod project_dialog;
mod project_info;
mod project_menu;

use config::Config;
use env_dialog::EnvDialog;
use env_select::EnvSelect;
pub use mainwindow::MainWindow;
use module_dialog::ModuleDialog;
use project_dialog::ProjectDialog;
use project_info::ProjectInfo;
use project_menu::ProjectMenu;

fn error_message(msg: &str) {
    fltk::dialog::message_title("错误");
    fltk::dialog::message_default(msg);
}
