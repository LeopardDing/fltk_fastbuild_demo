use super::error_message;
use super::message::*;
use super::module_dialog::ModuleDialog;
use super::params::*;
use super::project_info::ProjectInfo;
use super::Config;
use fltk::prelude::*;
use fltk::*;
use fltk_theme::*;

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct ProjectDialog {
    parent: window::Window,
    win: window::Window,
    receiver: app::Receiver<WindowMsg>,
    projectInfo: ProjectInfo,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl ProjectDialog {
    pub fn new(parent: window::Window, title: &str) -> Self {
        let (sender, receiver) = app::channel::<WindowMsg>();
        let mut win = window::Window::default()
            .with_pos(
                parent.x() + (parent.width() - PROJECT_DIALOG_SIZE.0) / 2,
                parent.y() + (parent.height() - PROJECT_DIALOG_SIZE.1) / 2,
            )
            .with_size(PROJECT_DIALOG_SIZE.0, PROJECT_DIALOG_SIZE.1)
            .with_label(title);
        win.make_resizable(true);

        let projectInfo = ProjectInfo::new(sender.clone(), true);

        win.end();
        win.make_modal(true);
        win.show();

        ProjectDialog {
            parent,
            win,
            receiver,
            projectInfo,
        }
    }

    pub fn run(&mut self) {
        while self.win.shown() {
            app::wait();
            match self.receiver.recv() {
                Some(WindowMsg::ProjectInfoMsg(ProjectInfoMsg::JustMoudule)) => {
                    if self.projectInfo.justModule.value() {
                        self.projectInfo.projectPath.deactivate();
                        self.projectInfo.btnProjectPath.deactivate();
                        self.projectInfo.buildCmd.deactivate();
                    } else {
                        self.projectInfo.projectPath.activate();
                        self.projectInfo.btnProjectPath.activate();
                        self.projectInfo.buildCmd.activate();
                    }
                }
                Some(WindowMsg::ProjectInfoMsg(ProjectInfoMsg::BtnProjectPath)) => {
                    let mut dialog =
                        dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseDir);
                    // dialog.set_directory(&std::path::Path::new("D:/CloudMusic"));
                    dialog.show();
                    println!("{:?}", dialog.filename());
                }
                Some(WindowMsg::ProjectInfoMsg(ProjectInfoMsg::BtnXmpPath)) => {}
                Some(WindowMsg::ProjectInfoMsg(ProjectInfoMsg::AddModule)) => {
                    ModuleDialog::new(self.parent.clone(), "添加组件").run();
                }
                Some(WindowMsg::ProjectInfoMsg(ProjectInfoMsg::ModifyModule)) => {}
                Some(WindowMsg::ProjectInfoMsg(ProjectInfoMsg::DelModule)) => {}
                Some(WindowMsg::ProjectInfoMsg(ProjectInfoMsg::ModuleLibsPath)) => {}
                Some(WindowMsg::ProjectInfoMsg(ProjectInfoMsg::BtnSave)) => {}
                Some(WindowMsg::ProjectInfoMsg(ProjectInfoMsg::BtnCancel)) => {
                    self.win.hide();
                }
                _ => {}
            }
        }
    }
}
