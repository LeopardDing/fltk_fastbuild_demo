use super::error_message;
use super::message::*;
use super::params::*;
use super::ProjectDialog;
use fltk::prelude::*;
use fltk::*;
use fltk_theme::*;

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct ProjectMenu {
    parent: window::Window,
    projectList: browser::Browser,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl ProjectMenu {
    pub fn init(
        parent: window::Window,
        mut flex: group::Flex,
        sender: app::Sender<WindowMsg>,
    ) -> Self {
        let mut col = group::Flex::default_fill().column();
        flex.set_size(&col, 180);

        let row = group::Flex::default_fill().row();
        col.set_size(&row, COMMON_SIZE.1);
        let mut btn = button::Button::default().with_label("修改工程");
        btn.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btn.emit(
            sender,
            WindowMsg::ProjectMenuMsg(ProjectMenuMsg::ModifyProject),
        );
        let mut btn = button::Button::default().with_label("删除工程");
        btn.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btn.emit(
            sender,
            WindowMsg::ProjectMenuMsg(ProjectMenuMsg::DelProject),
        );
        row.end();

        let mut btn = button::Button::default().with_label("新建工程");
        col.set_size(&btn, COMMON_SIZE.1);
        btn.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btn.emit(
            sender,
            WindowMsg::ProjectMenuMsg(ProjectMenuMsg::AddProject),
        );

        let mut projectList = browser::Browser::default();
        let widths = &[180];
        projectList.set_type(browser::BrowserType::Hold);
        projectList.set_frame(widget_themes::OS_TABS_BOX);
        projectList.set_column_widths(widths);
        projectList.set_column_char('\t');
        projectList.set_text_size(16);

        col.end();

        let mut line = frame::Frame::default();
        flex.set_size(&line, 1);
        line.draw(|f| {
            let (x1, y1) = (f.x(), f.y() - WIDGET_MARGIN);
            let (x2, y2) = (f.x(), f.y() + f.h() + WIDGET_MARGIN);
            draw::set_draw_color(enums::Color::Gray0.inactive());
            draw::draw_line(x1, y1, x2, y2);
        });

        ProjectMenu {
            parent,
            projectList,
        }
    }

    pub fn event(&mut self, msg: &ProjectMenuMsg) {
        match msg {
            ProjectMenuMsg::AddProject => {
                let mut dialog = ProjectDialog::new(self.parent.clone(), "新建工程");
                dialog.run();
            }
            ProjectMenuMsg::ModifyProject => {
                let items = self.projectList.selected_items();
                if items.is_empty() || items[0] <= 0 {
                    error_message("请选中要修改的工程");
                    return;
                }

                let mut dialog = ProjectDialog::new(self.parent.clone(), "修改工程");
                dialog.run();
            }
            ProjectMenuMsg::DelProject => {
                let items = self.projectList.selected_items();
                if items.is_empty() || items[0] <= 0 {
                    error_message("请选中要删除的工程");
                    return;
                }
            }
        }
    }
}
