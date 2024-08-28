use crate::domain::model::manifest::MANIFEST;

pub trait Act {
    fn location_action(&self) -> anyhow::Result<()> {
        let manifest = MANIFEST.expect("[ERROR] Failed to read declaration file.");
    }
}