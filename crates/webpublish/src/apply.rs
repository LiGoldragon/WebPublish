use std::io::{self, Write};

use crate::configuration::Configuration;
use crate::invocation::{InvocationResult, WranglerArgv};
use crate::wrangler::Wrangler;

#[derive(Debug, Clone)]
pub struct Apply {
    configuration: Configuration,
    wrangler: Wrangler,
}

impl Apply {
    pub fn from_configuration(configuration: Configuration) -> Self {
        Self {
            configuration,
            wrangler: Wrangler::from_path(),
        }
    }

    pub fn into_outcome(self) -> Result<ApplyOutcome, ApplyError> {
        let project_request = self.configuration.project_request();
        let project_invocation = self
            .wrangler
            .run(WranglerArgv::from_project(project_request))
            .map_err(ApplyError::from_io)?;
        self.write_invocation(&project_invocation)?;
        if !project_invocation.is_success() && !project_invocation.is_idempotent_failure() {
            return Err(ApplyError::from_invocation(project_invocation));
        }

        for domain_request in self
            .configuration
            .domain_request_list()
            .into_requests()
        {
            let domain_invocation = self
                .wrangler
                .run(WranglerArgv::from_domain(domain_request))
                .map_err(ApplyError::from_io)?;
            self.write_invocation(&domain_invocation)?;
            if !domain_invocation.is_success() && !domain_invocation.is_idempotent_failure() {
                return Err(ApplyError::from_invocation(domain_invocation));
            }
        }

        Ok(ApplyOutcome::success())
    }

    fn write_invocation(&self, invocation: &InvocationResult) -> Result<(), ApplyError> {
        let mut stdout = io::stdout();
        stdout
            .write_all(invocation.stdout())
            .map_err(ApplyError::from_io)?;
        stdout.flush().map_err(ApplyError::from_io)?;
        let mut stderr = io::stderr();
        stderr
            .write_all(invocation.stderr())
            .map_err(ApplyError::from_io)?;
        stderr.flush().map_err(ApplyError::from_io)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ApplyOutcome {
    exit_code: i32,
}

impl ApplyOutcome {
    pub fn from_configuration(configuration: Configuration) -> Result<Self, ApplyError> {
        Apply::from_configuration(configuration).into_outcome()
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }

    fn success() -> Self {
        Self { exit_code: 0 }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApplyError {
    #[error("wrangler invocation failed")]
    Invocation { result: InvocationResult },
    #[error("io error: {message}")]
    Io { message: String },
}

impl ApplyError {
    pub fn from_invocation(result: InvocationResult) -> Self {
        Self::Invocation { result }
    }

    pub fn from_io(error: io::Error) -> Self {
        Self::Io {
            message: error.to_string(),
        }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Invocation { result } => result.exit_code(),
            Self::Io { .. } => 1,
        }
    }
}
