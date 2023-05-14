#[derive(Copy, Clone)]
pub enum WindowMsg {
    EnvSelectMsg(EnvSelectMsg),
    ProjectMenuMsg(ProjectMenuMsg),
    EnvDialogMsg(EnvDialogMsg),
    ProjectInfoMsg(ProjectInfoMsg),
    ModuleDialogMsg(ModuleDialogMsg),
}

#[derive(Copy, Clone)]
pub enum EnvSelectMsg {
    SelectEnv,
    BtnEdit,
    BtnLight,
}

#[derive(Copy, Clone)]
pub enum ProjectMenuMsg {
    AddProject,
    ModifyProject,
    DelProject,
}

#[derive(Copy, Clone)]
pub enum EnvDialogMsg {
    BtnAdd,
    BtnDel,
}

#[derive(Copy, Clone)]
pub enum ProjectInfoMsg {
    JustMoudule,
    BtnProjectPath,
    BtnXmpPath,
    AddModule,
    ModifyModule,
    DelModule,
    ModuleLibsPath,
    BtnSave,
    BtnCancel,
}

#[derive(Copy, Clone)]
pub enum ModuleDialogMsg {
    ChoiceModule,
    BuildModule,
    ModulePath,
    BtnSave,
    BtnCancel,
}
