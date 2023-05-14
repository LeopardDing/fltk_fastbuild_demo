use super::message::*;
use super::params::*;
use super::Config;
use super::EnvDialog;
use fltk::prelude::*;
use fltk::*;
use fltk_theme::*;

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct EnvSelect {
    parent: window::Window,
    choiceEnv: menu::Choice,
    btnEdit: button::Button,
    btnLight: button::Button,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl EnvSelect {
    pub fn init(
        parent: window::Window,
        mut flex: group::Flex,
        sender: app::Sender<WindowMsg>,
    ) -> Self {
        let mut row = group::Flex::default_fill().row();
        flex.set_size(&row, TITLE_HEIGHT);

        let label = frame::Frame::default().with_label("选择编译环境: ");
        row.set_size(&label, 80);

        let mut choiceEnv = menu::Choice::default();
        row.set_size(&choiceEnv, 200);
        choiceEnv.emit(sender, WindowMsg::EnvSelectMsg(EnvSelectMsg::SelectEnv));
        choiceEnv.set_frame(widget_themes::OS_TABS_BOX);

        let mut btnEdit = button::Button::default().with_label("编辑");
        row.set_size(&btnEdit, COMMON_SIZE.0);
        btnEdit.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btnEdit.emit(sender, WindowMsg::EnvSelectMsg(EnvSelectMsg::BtnEdit));

        frame::Frame::default();

        let mut btnLight = button::Button::default().with_label("关灯");
        row.set_size(&btnLight, COMMON_SIZE.0);
        btnLight.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btnLight.emit(sender, WindowMsg::EnvSelectMsg(EnvSelectMsg::BtnLight));

        row.end();

        let mut line = frame::Frame::default();
        flex.set_size(&line, 1);
        line.draw(|f| {
            let (x1, y1) = (f.x() - WIDGET_MARGIN, f.y());
            let (x2, y2) = (f.width() + 5 + WIDGET_MARGIN, f.y());
            draw::set_draw_color(enums::Color::Gray0.inactive());
            draw::draw_line(x1, y1, x2, y2);
        });

        EnvSelect {
            choiceEnv,
            btnEdit,
            btnLight,
            parent,
        }
    }

    pub fn event(&mut self, msg: &EnvSelectMsg) {
        match msg {
            EnvSelectMsg::SelectEnv => {
                let index = self.choiceEnv.value();
                let mut info = Config::instance().getEnvInfo();
                info.0 = index;
                Config::instance().setEnvList(&info);
            }
            EnvSelectMsg::BtnEdit => {
                let mut dialog = EnvDialog::new(&self.parent);
                dialog.run();
                self.updateEnvInfo();
            }
            EnvSelectMsg::BtnLight => {
                let light = Config::instance().getLight();
                if light {
                    self.btnLight.set_label("开灯");
                    WidgetTheme::new(ThemeType::Dark).apply();
                } else {
                    self.btnLight.set_label("关灯");
                    WidgetTheme::new(ThemeType::AquaClassic).apply();
                }
                Config::instance().setLight(!light);
            }
        }
    }

    pub fn updateEnvInfo(&mut self) {
        let info = Config::instance().getEnvInfo();
        let list = &info.1;
        let mut index = info.0;

        self.choiceEnv.clear();
        for item in list {
            self.choiceEnv
                .add_choice(&format!("{}@@{}", item.1, item.0));
        }

        if list.len() > 0 && index < 0 {
            index = 0;
            Config::instance().setEnvList(&(index.clone(), list.to_owned()));
        } else if list.len() == 0 && index >= 0 {
            index = -1;
            Config::instance().setEnvList(&(index.clone(), list.to_owned()));
        }

        self.choiceEnv.set_value(index);
        self.choiceEnv.redraw();
    }
}
