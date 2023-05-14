use super::error_message;
use super::message::*;
use super::params::*;
use super::Config;
use fltk::prelude::*;
use fltk::*;
use fltk_theme::*;

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct ProjectInfo {
    pub justModule: button::CheckButton,
    pub projectName: input::Input,
    pub projectPath: input::Input,
    pub btnProjectPath: button::Button,
    pub buildCmd: input::MultilineInput,
    pub xmlPath: input::Input,
    pub btnXmlPath: button::Button,
    pub moduleList: browser::HoldBrowser,
    pub addModule: button::Button,
    pub modifyModule: button::Button,
    pub delModule: button::Button,
    pub moduleLibPath: input::Input,
    pub btnLibPath: button::Button,
    pub btnSave: button::Button,
    pub btnCancel: button::Button,
    pub isEdit: bool,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl ProjectInfo {
    pub fn new(sender: app::Sender<WindowMsg>, isEdit: bool) -> Self {
        let mut col = group::Flex::default_fill().column();
        col.set_margin(WIDGET_MARGIN);

        let mut row = group::Flex::default_fill().row();
        col.set_size(&row, COMMON_SIZE.1);
        let frame = frame::Frame::default();
        row.set_size(&frame, LABEL_WIDTH);
        let mut justModule = button::CheckButton::default().with_label("仅编译组件");
        justModule.emit(
            sender,
            WindowMsg::ProjectInfoMsg(ProjectInfoMsg::JustMoudule),
        );
        if !isEdit {
            justModule.deactivate();
        }
        row.end();

        let mut line = frame::Frame::default();
        col.set_size(&line, 1);
        line.draw(|f| {
            let (x1, y1) = (f.x(), f.y());
            let (x2, y2) = (f.x() + f.w(), f.y());
            draw::set_draw_color(enums::Color::Gray0.inactive());
            draw::draw_line(x1, y1, x2, y2);
        });

        let mut row = group::Flex::default_fill().row();
        col.set_size(&row, COMMON_SIZE.1);
        let frame = frame::Frame::default();
        row.set_size(&frame, LABEL_WIDTH);
        let mut projectName = input::Input::default().with_label("工程名称: ");
        projectName.set_frame(widget_themes::OS_INPUT_THIN_DOWN_BOX);
        let mut frame = frame::Frame::default();
        row.set_size(&frame, COMMON_SIZE.0);
        if !isEdit {
            // projectName.set_readonly(true);
            frame.hide();
        }
        row.end();

        let mut row = group::Flex::default_fill().row();
        col.set_size(&row, COMMON_SIZE.1);
        let frame = frame::Frame::default();
        row.set_size(&frame, LABEL_WIDTH);
        let mut projectPath = input::Input::default().with_label("打包环境: ");
        projectPath.set_frame(widget_themes::OS_INPUT_THIN_DOWN_BOX);
        let mut btnProjectPath = button::Button::default().with_label("选择目录");
        btnProjectPath.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btnProjectPath.emit(
            sender,
            WindowMsg::ProjectInfoMsg(ProjectInfoMsg::BtnProjectPath),
        );
        row.set_size(&btnProjectPath, COMMON_SIZE.0);
        if !isEdit {
            // projectPath.set_readonly(true);
            btnProjectPath.hide();
        }
        row.end();

        let mut row = group::Flex::default_fill().row();
        col.set_size(&row, COMMON_SIZE.1 * 3);
        let frame = frame::Frame::default();
        row.set_size(&frame, LABEL_WIDTH);
        let mut buildCmd = input::MultilineInput::default().with_label("打包命令: ");
        buildCmd.set_frame(widget_themes::OS_INPUT_THIN_DOWN_BOX);
        buildCmd.set_wrap(true);
        let mut frame = frame::Frame::default();
        row.set_size(&frame, COMMON_SIZE.0);
        if !isEdit {
            // buildCmd.set_readonly(true);
            frame.hide();
        }
        row.end();

        let mut line = frame::Frame::default();
        col.set_size(&line, 1);
        line.draw(|f| {
            let (x1, y1) = (f.x(), f.y());
            let (x2, y2) = (f.x() + f.w(), f.y());
            draw::set_draw_color(enums::Color::Gray0.inactive());
            draw::draw_line(x1, y1, x2, y2);
        });

        let mut row = group::Flex::default_fill().row();
        col.set_size(&row, COMMON_SIZE.1);
        let frame = frame::Frame::default();
        row.set_size(&frame, LABEL_WIDTH);
        let mut xmlPath = input::Input::default().with_label("XML路径: ");
        xmlPath.set_frame(widget_themes::OS_INPUT_THIN_DOWN_BOX);
        let mut btnXmlPath = button::Button::default().with_label("选择文件");
        btnXmlPath.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btnXmlPath.emit(
            sender,
            WindowMsg::ProjectInfoMsg(ProjectInfoMsg::BtnXmpPath),
        );
        row.set_size(&btnXmlPath, COMMON_SIZE.0);
        if !isEdit {
            // xmlPath.set_readonly(true);
            btnXmlPath.hide();
        }
        row.end();

        let mut row = group::Flex::default_fill().row();
        let frame = frame::Frame::default().with_label("组件列表: ");
        row.set_size(&frame, LABEL_WIDTH);
        let mut moduleList = browser::HoldBrowser::default();
        let widths = &[20, 50, 150, 500];
        moduleList.set_frame(widget_themes::OS_TABS_BOX);
        moduleList.set_text_size(14);
        moduleList.set_column_widths(widths);
        moduleList.set_column_char('\t');
        moduleList.add("@B49\t@B49@b使能\t@B49@b组件名称\t@B49@b代码路径");

        let mut col1 = group::Flex::default_fill().column();
        row.set_size(&col1, COMMON_SIZE.0);
        let mut frame = frame::Frame::default();
        let mut addModule = button::Button::default().with_label("添加组件");
        addModule.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        addModule.emit(sender, WindowMsg::ProjectInfoMsg(ProjectInfoMsg::AddModule));
        col1.set_size(&addModule, COMMON_SIZE.1);
        let mut modifyModule = button::Button::default().with_label("修改组件");
        modifyModule.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        modifyModule.emit(
            sender,
            WindowMsg::ProjectInfoMsg(ProjectInfoMsg::ModifyModule),
        );
        col1.set_size(&modifyModule, COMMON_SIZE.1);
        let mut delModule = button::Button::default().with_label("删除组件");
        delModule.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        delModule.emit(sender, WindowMsg::ProjectInfoMsg(ProjectInfoMsg::DelModule));
        col1.set_size(&delModule, COMMON_SIZE.1);
        frame::Frame::default();
        col1.end();
        if !isEdit {
            col1.hide();
        }
        row.end();

        let mut row = group::Flex::default_fill().row();
        col.set_size(&row, COMMON_SIZE.1);
        let frame = frame::Frame::default();
        row.set_size(&frame, LABEL_WIDTH);
        let mut moduleLibPath = input::Input::default().with_label("组件库路径: ");
        moduleLibPath.set_frame(widget_themes::OS_INPUT_THIN_DOWN_BOX);
        let mut btnLibPath = button::Button::default().with_label("选择目录");
        btnLibPath.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btnLibPath.emit(
            sender,
            WindowMsg::ProjectInfoMsg(ProjectInfoMsg::ModuleLibsPath),
        );
        row.set_size(&btnLibPath, COMMON_SIZE.0);
        if !isEdit {
            // moduleLibPath.set_readonly(true);
            btnLibPath.hide();
        }
        row.end();

        let mut line = frame::Frame::default();
        col.set_size(&line, 20);
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
        row.set_size(&btnSave, COMMON_SIZE.0 + 30);
        btnSave.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btnSave.emit(sender, WindowMsg::ProjectInfoMsg(ProjectInfoMsg::BtnSave));
        let frame = frame::Frame::default();
        row.set_size(&frame, COMMON_SIZE.0 * 2);
        let mut btnCancel = button::Button::default().with_label("取消");
        row.set_size(&btnCancel, COMMON_SIZE.0 + 30);
        btnCancel.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
        btnCancel.emit(sender, WindowMsg::ProjectInfoMsg(ProjectInfoMsg::BtnCancel));
        frame::Frame::default();
        row.end();

        let mut frame = frame::Frame::default();
        col.set_size(&frame, 10);

        if !isEdit {
            line.hide();
            row.hide();
            frame.hide();
        }

        col.end();

        ProjectInfo {
            justModule,
            projectName,
            projectPath,
            btnProjectPath,
            buildCmd,
            xmlPath,
            btnXmlPath,
            moduleList,
            addModule,
            modifyModule,
            delModule,
            moduleLibPath,
            btnLibPath,
            btnSave,
            btnCancel,
            isEdit,
        }
    }
}
