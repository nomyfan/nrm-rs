pub(crate) const NPMRC_HOME: &str = "home";
pub(crate) const NPMRC_URL: &str = "registry";

#[derive(Debug, Clone)]
pub(crate) struct NpmRegistry {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) in_use: bool,
    pub(crate) home: Option<String>,
    pub(crate) kvs: Option<Vec<(String, String)>>,
}

impl NpmRegistry {
    pub(crate) fn new<S>(name: S, url: S, home: Option<S>) -> Self
    where
        S: AsRef<str>,
    {
        NpmRegistry {
            name: name.as_ref().into(),
            in_use: false,
            url: url.as_ref().into(),
            home: home.map(|s| s.as_ref().into()),
            kvs: None,
        }
    }
}

pub(crate) fn get_preset_registries() -> Vec<NpmRegistry> {
    vec![
        NpmRegistry::new(
            "npm",
            "https://registry.npmjs.org/",
            Some("https://www.npmjs.org"),
        ),
        NpmRegistry::new(
            "yarn",
            "https://registry.yarnpkg.com/",
            Some("https://yarnpkg.com"),
        ),
    ]
}
