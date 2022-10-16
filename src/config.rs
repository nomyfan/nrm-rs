use std::vec::IntoIter;

pub(crate) const NPMRC_HOME: &str = "home";
pub(crate) const NPMRC_URL: &str = "registry";

pub(crate) type KV = (String, String);
pub(crate) struct NPMRC {
    inner: Vec<KV>,
}

impl NPMRC {
    pub(crate) fn new() -> Self {
        NPMRC { inner: vec![] }
    }

    pub(crate) fn from_into_iter<S>(into_iter: S) -> Self
    where
        S: IntoIterator<Item = KV>,
    {
        let mut npmrc = NPMRC::new();
        for (key, value) in into_iter {
            npmrc.insert(key, value);
        }

        npmrc
    }

    pub(crate) fn insert<K, V>(&mut self, key: K, value: V) -> Option<KV>
    where
        K: Into<String> + AsRef<str>,
        V: Into<String> + AsRef<str>,
    {
        match self.inner.iter().position(|x| &x.0[..] == key.as_ref()) {
            Some(nth) => Some(std::mem::replace(
                &mut self.inner[nth],
                (key.into(), value.into()),
            )),
            None => {
                self.inner.push((key.into(), value.into()));
                None
            }
        }
    }

    pub(crate) fn append(&mut self, npmrc: &mut NPMRC) -> Self {
        let mut replaced = vec![];
        for (key, value) in npmrc.inner.drain(..) {
            if let Some(kv) = self.insert(key, value) {
                replaced.push(kv);
            }
        }

        NPMRC::from_into_iter(replaced)
    }
}

impl IntoIterator for NPMRC {
    type Item = KV;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NpmRegistry {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) in_use: bool,
    pub(crate) home: Option<String>,
    pub(crate) kvs: Option<Vec<(String, String)>>,
}

#[allow(clippy::from_over_into)]
impl Into<NPMRC> for NpmRegistry {
    fn into(self) -> NPMRC {
        let Self { url, home, kvs, .. } = self;
        let mut npmrc = NPMRC::new();
        npmrc.insert(NPMRC_URL, url);

        if let Some(home) = home {
            npmrc.insert(NPMRC_HOME, &home);
        }

        if let Some(kvs) = kvs {
            npmrc.append(&mut NPMRC::from_into_iter(kvs));
        }

        npmrc
    }
}

impl NpmRegistry {
    pub(crate) fn new<N, U, H>(name: N, url: U, home: Option<H>) -> Self
    where
        N: Into<String>,
        U: Into<String>,
        H: Into<String>,
    {
        NpmRegistry {
            name: name.into(),
            in_use: false,
            url: url.into(),
            home: home.map(|s| s.into()),
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
