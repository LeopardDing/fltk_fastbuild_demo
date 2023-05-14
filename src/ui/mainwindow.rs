use super::message::*;
use super::params::*;
use super::Config;
use super::EnvSelect;
use super::ProjectInfo;
use super::ProjectMenu;
use fltk::prelude::*;
use fltk::*;
use fltk_theme::*;

pub struct MainWindow {
    app: app::App,
    receiver: app::Receiver<WindowMsg>,
    title: EnvSelect,
    menu: ProjectMenu,
}

impl MainWindow {
    pub fn new() -> Self {
        let (sender, receiver) = app::channel::<WindowMsg>();
        let app = app::App::default();
        app::set_font_size(12);
        WidgetTheme::new(if Config::instance().getLight() {
            ThemeType::AquaClassic
        } else {
            ThemeType::Dark
        })
        .apply();

        let mut win = window::Window::default()
            .with_size(WINDOWS_SIZE.0, WINDOWS_SIZE.1)
            .with_label("工程配置工具")
            .center_screen();
        win.resizable(&win);

        let mut col = group::Flex::default_fill().column();
        col.set_margin(WIDGET_MARGIN);
        let mut title = EnvSelect::init(win.clone(), col.clone(), sender.clone());
        let row = group::Flex::default_fill().row();
        let mut menu = ProjectMenu::init(win.clone(), row.clone(), sender.clone());

        ProjectInfo::new(sender.clone(), false);

        row.end();
        col.end();
        win.end();
        win.show();

        title.updateEnvInfo();

        MainWindow {
            app,
            receiver,
            title,
            menu,
        }
    }

    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(val) = self.receiver.recv() {
                match val {
                    WindowMsg::EnvSelectMsg(msg) => self.title.event(&msg),
                    WindowMsg::ProjectMenuMsg(msg) => self.menu.event(&msg),
                    _ => {}
                }
            }
        }
    }
}
