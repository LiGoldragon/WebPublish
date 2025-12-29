#[derive(Debug, Clone)]
pub struct Configuration {
    pub site: SiteIdentity,
    pub source: Source,
    pub pages: Pages,
    pub domains: DomainBindings,
}

impl Configuration {
    pub fn project_request(&self) -> ProjectRequest {
        ProjectRequest {
            project_name: self.pages.project_name.clone(),
            production_branch: self.pages.production_branch.clone(),
            owner: self.source.owner.clone(),
            repository: self.source.repository.clone(),
            build_command: self.pages.build_command.clone(),
            build_output_directory: self.pages.build_output_directory.clone(),
            account_id: self.pages.account_id.clone(),
        }
    }

    pub fn domain_request_list(&self) -> DomainRequestList {
        let mut requests = Vec::new();
        if let Some(primary_domain) = self.domains.primary_domain.clone() {
            requests.push(DomainRequest {
                project_name: self.pages.project_name.clone(),
                domain: primary_domain,
                account_id: self.pages.account_id.clone(),
            });
        }
        for domain in &self.domains.alternate_domains {
            requests.push(DomainRequest {
                project_name: self.pages.project_name.clone(),
                domain: domain.clone(),
                account_id: self.pages.account_id.clone(),
            });
        }
        DomainRequestList { requests }
    }
}

#[derive(Debug, Clone)]
pub struct SiteIdentity {
    pub stable_id: String,
    pub display_name: String,
}

#[derive(Debug, Clone)]
pub struct Source {
    pub owner: String,
    pub repository: String,
}

#[derive(Debug, Clone)]
pub struct Pages {
    pub project_name: String,
    pub production_branch: String,
    pub account_id: Option<String>,
    pub build_command: String,
    pub build_output_directory: String,
}

#[derive(Debug, Clone)]
pub struct DomainBindings {
    pub primary_domain: Option<String>,
    pub alternate_domains: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ProjectRequest {
    pub project_name: String,
    pub production_branch: String,
    pub owner: String,
    pub repository: String,
    pub build_command: String,
    pub build_output_directory: String,
    pub account_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DomainRequestList {
    requests: Vec<DomainRequest>,
}

impl DomainRequestList {
    pub fn into_requests(self) -> Vec<DomainRequest> {
        self.requests
    }
}

#[derive(Debug, Clone)]
pub struct DomainRequest {
    pub project_name: String,
    pub domain: String,
    pub account_id: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigurationError {
    #[error("Cap'n Proto decode error: {message}")]
    Decode { message: String },
    #[error("Cap'n Proto message root error: {message}")]
    Root { message: String },
}

impl ConfigurationError {
    pub fn from_decode(message: String) -> Self {
        Self::Decode { message }
    }

    pub fn from_root(message: String) -> Self {
        Self::Root { message }
    }
}
