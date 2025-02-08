pub struct DashboardBase {
    pub sidebar: DashboardSidebar,
    pub header: DashboardHeader,
    pub content: String, // TODO: implement this to be askama compatible
}

pub struct DashboardSidebar;

pub struct DashboardHeader;

pub struct DashboardExtension {
    base: DashboardBase,
    extensions: Vec<String>, // TODO: implement this
}

impl DashboardExtension {
    pub fn register(&self, extension: String) -> &Self {
        // TODO: implement this
        self
    }
}
