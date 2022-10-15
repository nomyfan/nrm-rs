pub(crate) const KEY_HOME: &str = "home";
pub(crate) const KEY_REGISTRY: &str = "registry";

#[derive(Debug, Clone)]
pub(crate) struct NpmRegistry {
    pub(crate) name: String,
    pub(crate) registry: String,
    pub(crate) in_use: bool,
    pub(crate) home: Option<String>,
}

impl NpmRegistry {
    pub(crate) fn new<S>(name: S, registry: S, home: Option<S>) -> Self
    where
        S: AsRef<str>,
    {
        NpmRegistry {
            name: name.as_ref().into(),
            in_use: false,
            registry: registry.as_ref().into(),
            home: home.map(|s| s.as_ref().into()),
        }
    }
}

pub(crate) fn get_embed_registries() -> Vec<NpmRegistry> {
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
