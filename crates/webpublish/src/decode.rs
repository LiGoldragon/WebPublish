use capnp::message::ReaderOptions;
use capnp::serialize_packed;
use std::io;

use crate::configuration::{
    Configuration, ConfigurationError, DomainBindings, Pages, SiteIdentity, Source,
};
use crate::stdin_bytes::StdinBytes;
use crate::webpublish_capnp::web_publish_configuration;

impl Configuration {
    pub fn from_bytes(bytes: StdinBytes) -> Result<Self, ConfigurationError> {
        let mut cursor = io::Cursor::new(bytes.into_bytes());
        let message = serialize_packed::read_message(&mut cursor, ReaderOptions::new())
            .map_err(|error| ConfigurationError::from_decode(error.to_string()))?;
        let root: web_publish_configuration::Reader = message
            .get_root()
            .map_err(|error| ConfigurationError::from_root(error.to_string()))?;
        Self::from_reader(root)
    }

    fn from_reader(reader: web_publish_configuration::Reader) -> Result<Self, ConfigurationError> {
        let site_reader = reader
            .get_site()
            .map_err(|error| ConfigurationError::from_decode(error.to_string()))?;
        let source_reader = reader
            .get_source()
            .map_err(|error| ConfigurationError::from_decode(error.to_string()))?;
        let pages_reader = reader
            .get_pages()
            .map_err(|error| ConfigurationError::from_decode(error.to_string()))?;
        let domains_reader = reader
            .get_domains()
            .map_err(|error| ConfigurationError::from_decode(error.to_string()))?;

        let site = SiteIdentity {
            stable_id: site_reader
                .get_stable_id()
                .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
                .to_string(),
            display_name: site_reader
                .get_display_name()
                .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
                .to_string(),
        };

        let source = Source {
            owner: source_reader
                .get_owner()
                .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
                .to_string(),
            repository: source_reader
                .get_repository()
                .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
                .to_string(),
        };

        let account_id = match pages_reader
            .get_account_id()
            .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
            .which()
            .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
        {
            crate::webpublish_capnp::cloudflare_pages_project::account_id::Which::Absent(()) => {
                None
            }
            crate::webpublish_capnp::cloudflare_pages_project::account_id::Which::Value(value) => {
                Some(
                    value
                        .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
                        .to_string(),
                )
            }
        };

        let pages = Pages {
            project_name: pages_reader
                .get_project_name()
                .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
                .to_string(),
            production_branch: pages_reader
                .get_production_branch()
                .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
                .to_string(),
            account_id,
            build_command: pages_reader
                .get_build_command()
                .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
                .to_string(),
            build_output_directory: pages_reader
                .get_build_output_directory()
                .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
                .to_string(),
        };

        let primary_domain = match domains_reader
            .get_primary_domain()
            .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
            .which()
            .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
        {
            crate::webpublish_capnp::domain_bindings::primary_domain::Which::Absent(()) => None,
            crate::webpublish_capnp::domain_bindings::primary_domain::Which::Domain(domain) => {
                Some(
                    domain
                        .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
                        .to_string(),
                )
            }
        };

        let alternates_reader = domains_reader
            .get_alternate_domains()
            .map_err(|error| ConfigurationError::from_decode(error.to_string()))?;
        let mut alternate_domains = Vec::with_capacity(alternates_reader.len() as usize);
        for index in 0..alternates_reader.len() {
            alternate_domains.push(
                alternates_reader
                    .get(index)
                    .map_err(|error| ConfigurationError::from_decode(error.to_string()))?
                    .to_string(),
            );
        }

        let domains = DomainBindings {
            primary_domain,
            alternate_domains,
        };

        Ok(Self {
            site,
            source,
            pages,
            domains,
        })
    }
}
