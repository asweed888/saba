use crate::domain::model::manifest::{Manifest, Lang, Arch};
use yaml_rust::YamlLoader;
use anyhow::anyhow;
use std::fs::read_to_string;



pub struct ManifestRepository {
    pub manifest: Manifest,
}

impl ManifestRepository {
    pub fn new() -> anyhow::Result<Self> {
        let file = read_to_string("./saba.yml");
        let s = file.map_err(|_| anyhow!("Failed to load saba.yml"))?.to_string();
        let file_content = YamlLoader::load_from_str(&s)
            .map_err(|_| anyhow!("Failed to load saba.yml"))?;
        let manifest = file_content.get(0).clone()
            .ok_or_else(|| anyhow!("Failed to load saba.yml"))?;

        let lang = manifest["lang"]
            .as_str()
            .ok_or_else(|| anyhow!("Failed to get lang from manifest"))?;

        let arch = manifest["arch"]
            .as_str()
            .unwrap_or("plain");

        let root = manifest["root"]
            .as_str()
            .unwrap_or("");

        let spec = manifest["spec"]
            .as_vec()
            .ok_or_else(|| anyhow!("Failed to get spec from manifest"))?
            .clone();

        let lang = Lang::from_rawdata(lang)?;
        let arch = Arch::from_rawdata(arch)?;

        let root = match root {
            "" => lang.default_root().to_string(),
            _ => root.to_string(),
        };

        Ok(Self{
            manifest: Manifest{
                lang,
                arch,
                root,
                spec,
            },
        })
    }
}

