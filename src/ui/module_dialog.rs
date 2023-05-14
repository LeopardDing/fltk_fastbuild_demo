use super::error_message;
use super::message::*;
use super::params::*;
use super::Config;
use fltk::prelude::*;
use fltk::*;
use fltk_theme::*;

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct ModuleDialog {
    parent: window::Window,
    win: window::Window,
    receiver: app::Receiver<WindowMsg>,
    choiceModule: menu::Choice,
    buildModule: button::CheckButton,
    modulePath: input::Input,
    btnModulePath: button::Button,
    buildCmd: input::MultilineInput,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl ModuleDialog {
    pub fn new(parent: window::Window, title: &str) -> Self {
        let (sender, receiver) = app::channel::<WindowMsg>();
        let mut win = window::Window::default()
            .with_pos(
                parent.x() + (parent.width() - MODULE_DIALOG_SIZE.0) / 2,
                parent.y() + (parent.height() - MODULE_DIALOG_SIZE.1) / 2,
            )
            .with_size(MODULE_DIALOG_SIZE.0, MODULE_DIALOG_SIZE.1)
            .with_label(title);
        win.make_resizable(true);
        let mut col = group::Flex::default_fill().column();
        col.set_margin(WIDGET_MARGIN);

        let mut row = group::Flex::default_fill().row();
        col.set_size(&row, COMMON_SIZE.1);
        let frame = frame::Frame::default();
        row.set_size(&frame, LABEL_WIDTH);
        let mut choiceModule = menu::Choice::default().with_label("选择组建: ");
        choiceModule.set_frame(widget_themes::OS_TABS_BOX);
        choiceModule.emit(
            sender,
            WindowMsg::ModuleDialogMsg(ModuleDialogMsg::ChoiceModule),
        );
        let mut buildModule = button::CheckButton::default().with_label("是否编译");
        buildModule.emit(
            sender,
            WindowMsg::ModuleDialogMsg(ModuleDialogMsg::BuildModule),
        );
        row.set_size(&buildModule, LABEL_WIDTH);
        row.end();

        let mut row = group::Flex::default_fill().row();
        col.set_size(&row, COMMON_SIZE.1);
        let frame = frame::Frame::default();
        row.set_size(&frame, LABEL_WIDTH);
        let mut modulePath = input::Input::default().with_label("代码路径: ");
        modulePath.set_frame(widget_themes::OS_INPUT_THIN_DOWN_BOX);
        let mut btnModulePath = button::Button::default().with_label("选择目录");
        btnModulePath.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btnModulePath.emit(
            sender,
            WindowMsg::ModuleDialogMsg(ModuleDialogMsg::ModulePath),
        );
        row.set_size(&btnModulePath, COMMON_SIZE.0);
        row.end();

        let mut row = group::Flex::default_fill().row();
        let frame = frame::Frame::default();
        row.set_size(&frame, LABEL_WIDTH);
        let mut buildCmd = input::MultilineInput::default().with_label("编译命令: ");
        buildCmd.set_frame(widget_themes::OS_INPUT_THIN_DOWN_BOX);
        buildCmd.set_wrap(true);
        row.end();

        let mut line = frame::Frame::default();
        col.set_size(&line, 10);
        line.draw(|f| {
            let (x1, y1) = (f.x(), f.y());
            let (x2, y2) = (f.x() + f.w(), f.y());
            draw::set_draw_color(enums::Color::Gray0.inactive());
            draw::draw_line(x1, y1, x2, y2);
        });

        let mut row = group::Flex::default_fill().row();
        col.set_size(&row, COMMON_SIZE.1);
        frame::Frame::default();
        let mut btnSave = button::Button::default().with_label("保存");
        row.set_size(&btnSave, COMMON_SIZE.0);
        btnSave.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btnSave.emit(sender, WindowMsg::ModuleDialogMsg(ModuleDialogMsg::BtnSave));
        let frame = frame::Frame::default();
        row.set_size(&frame, COMMON_SIZE.0 + 10);
        let mut btnCancel = button::Button::default().with_label("取消");
        row.set_size(&btnCancel, COMMON_SIZE.0);
        btnCancel.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btnCancel.emit(
            sender,
            WindowMsg::ModuleDialogMsg(ModuleDialogMsg::BtnCancel),
        );
        frame::Frame::default();
        row.end();

        let frame = frame::Frame::default();
        col.set_size(&frame, 10);

        col.end();
        win.end();
        win.make_modal(true);
        win.show();

        ModuleDialog {
            parent,
            win,
            receiver,
            choiceModule,
            buildModule,
            modulePath,
            btnModulePath,
            buildCmd,
        }
    }

    pub fn run(&mut self) {
        while self.win.shown() {
            app::wait();
            match self.receiver.recv() {
                Some(WindowMsg::ModuleDialogMsg(ModuleDialogMsg::ChoiceModule)) => {}
                Some(WindowMsg::ModuleDialogMsg(ModuleDialogMsg::BuildModule)) => {
                    if self.buildModule.value() {
                        self.choiceModule.activate();
                        self.modulePath.activate();
                        self.btnModulePath.activate();
                        self.buildCmd.activate();
                    } else {
                        self.choiceModule.deactivate();
                        self.modulePath.deactivate();
                        self.btnModulePath.deactivate();
                        self.buildCmd.deactivate();
                    }
                }
                Some(WindowMsg::ModuleDialogMsg(ModuleDialogMsg::ModulePath)) => {}
                Some(WindowMsg::ModuleDialogMsg(ModuleDialogMsg::BtnSave)) => {}
                Some(WindowMsg::ModuleDialogMsg(ModuleDialogMsg::BtnCancel)) => {
                    self.win.hide();
                }
                _ => {}
            }
        }
    }
}
