use super::error_message;
use super::message::*;
use super::params::*;
use super::Config;
use fltk::prelude::*;
use fltk::*;
use fltk_theme::*;

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct EnvDialog {
    win: window::Window,
    receiver: app::Receiver<WindowMsg>,
    id: input::IntInput,
    address: input::Input,
    list: browser::HoldBrowser,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl EnvDialog {
    pub fn new(parent: &window::Window) -> Self {
        let (sender, receiver) = app::channel::<WindowMsg>();
        let mut win = window::Window::default()
            .with_pos(
                parent.x() + (parent.width() - ENV_DIALOG_SIZE.0) / 2,
                parent.y() + (parent.height() - ENV_DIALOG_SIZE.1) / 2,
            )
            .with_size(ENV_DIALOG_SIZE.0, ENV_DIALOG_SIZE.1)
            .with_label("信息编辑");
        win.make_resizable(true);

        let mut col = group::Flex::default_fill().column();
        col.set_margin(WIDGET_MARGIN);

        let mut row1 = group::Flex::default_fill().row();
        col.set_size(&row1, COMMON_SIZE.1);
        let label = frame::Frame::default().with_label("");
        row1.set_size(&label, 65);
        let mut id = input::IntInput::default().with_label("域账号: ");
        id.set_frame(widget_themes::OS_INPUT_THIN_DOWN_BOX);
        let mut btnAdd = button::Button::default().with_label("添加");
        row1.set_size(&btnAdd, 60);
        btnAdd.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btnAdd.emit(sender, WindowMsg::EnvDialogMsg(EnvDialogMsg::BtnAdd));
        row1.end();

        let mut row2 = group::Flex::default_fill().row();
        col.set_size(&row2, COMMON_SIZE.1);
        let label = frame::Frame::default().with_label("");
        row2.set_size(&label, 65);
        let mut address = input::Input::default().with_label("服务器地址: ");
        address.set_frame(widget_themes::OS_INPUT_THIN_DOWN_BOX);
        let mut btnDel = button::Button::default().with_label("删除");
        row2.set_size(&btnDel, 60);
        btnDel.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btnDel.emit(sender, WindowMsg::EnvDialogMsg(EnvDialogMsg::BtnDel));
        row2.end();

        let data = Config::instance().getEnvInfo().1;
        let mut list = browser::HoldBrowser::default();
        let widths = &[40, 150, 150];
        list.set_frame(widget_themes::OS_TABS_BOX);
        list.set_text_size(14);
        list.set_column_widths(widths);
        list.set_column_char('\t');
        list.add("@B49\t@B49@b服务器地址\t@B49@b域账号");
        for item in &data {
            list.add(&format!("\t{}\t{}", item.0, item.1));
        }

        col.end();
        win.end();
        win.make_modal(true);
        win.show();

        EnvDialog {
            win,
            receiver,
            id,
            address,
            list,
        }
    }

    pub fn run(&mut self) {
        while self.win.shown() {
            app::wait();
            if let Some(val) = self.receiver.recv() {
                match val {
                    WindowMsg::EnvDialogMsg(EnvDialogMsg::BtnAdd) => self.addBtnCallback(),
                    WindowMsg::EnvDialogMsg(EnvDialogMsg::BtnDel) => self.delBtnCallback(),
                    _ => {}
                }
            }
        }
    }

    fn addBtnCallback(&mut self) {
        if self.id.value().is_empty() {
            error_message("请输入域账号");
            return;
        }

        if self.address.value().is_empty() {
            error_message("请输入服务器地址");
            return;
        }

        let mut data = Config::instance().getEnvInfo();
        for item in &data.1 {
            if self.address.value() == item.0 && self.id.value() == item.1 {
                error_message("请勿重复添加");
                return;
            }
        }

        data.1.push((self.address.value(), self.id.value()));
        self.list
            .add(&format!("\t{}\t{}", self.address.value(), self.id.value()));

        Config::instance().setEnvList(&data);
    }

    fn delBtnCallback(&mut self) {
        let items = self.list.selected_items();
        if items.is_empty() || items[0] <= 1 {
            error_message("请选择要删除的内容");
            return;
        }

        let mut data = Config::instance().getEnvInfo();
        data.1.remove(items[0] as usize - 2);
        self.list.remove(items[0]);
        Config::instance().setEnvList(&data);
    }
}
