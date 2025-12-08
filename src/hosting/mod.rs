use anyhow::Result;

use crate::model::{Hosting, HostingProvider, SporeConfig};

mod cloudflare;
use cloudflare::CloudflarePages;

impl Hosting {
    pub fn apply(&self, config: &SporeConfig) -> Result<()> {
        match self.provider {
            HostingProvider::CloudflarePages => CloudflarePages::apply(config),
            HostingProvider::LocalStatic => {
                // placeholder for future local static hosting
                Ok(())
            }
            HostingProvider::S3Static => {
                // placeholder
                Ok(())
            }
            HostingProvider::CriomosHost => {
                // placeholder
                Ok(())
            }
        }
    }
}
