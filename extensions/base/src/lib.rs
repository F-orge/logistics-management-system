pub struct ExtensionNavigation {
    pub name: String,
    pub items: Vec<ExtensionNavigationItem>,
}

pub struct ExtensionNavigationItem {
    pub name: String, // name of the nav. example: overview
    pub path: String, // path of the navigation
    pub children: Option<ExtensionNavigation>,
}

pub trait Extension {
    // extension name
    fn name(&self) -> String;

    // navigation helper
    fn navigation(&self) -> ExtensionNavigation;

    // routes that will be implemented in the system.
    fn router(&self) -> axum::Router;
}

pub struct Extensions<T: Extension> {
    pub extensions: Vec<T>,
}

#[cfg(test)]
mod test {
    use axum::routing::get;

    use crate::{Extension, ExtensionNavigationItem};

    pub struct SampleExtension {}

    async fn hello_world() -> String {
        return "hello world".into();
    }
    impl Extension for SampleExtension {
        fn name(&self) -> String {
            "Sample Extension".into()
        }

        fn navigation(&self) -> crate::ExtensionNavigation {
            crate::ExtensionNavigation {
                name: "Sample Extension".into(),
                items: vec![ExtensionNavigationItem {
                    name: "Overview".into(),
                    path: "/".into(),
                    children: None,
                }],
            }
        }

        fn router(&self) -> axum::Router {
            axum::Router::new().route("/", get(hello_world))
        }
    }
}
