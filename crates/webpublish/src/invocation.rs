use std::process::{ExitStatus, Output};

use crate::configuration::{DomainRequest, ProjectRequest};

#[derive(Debug, Clone)]
pub struct WranglerArgv {
    argv: Vec<String>,
}

impl WranglerArgv {
    pub fn from_project(request: ProjectRequest) -> Self {
        let mut argv = vec![
            "pages".to_string(),
            "project".to_string(),
            "create".to_string(),
            request.project_name,
            format!("--production-branch={}", request.production_branch),
            "--source=github".to_string(),
            format!("--repo={}/{}", request.owner, request.repository),
            format!("--build-command={}", request.build_command),
            format!("--build-output={}", request.build_output_directory),
        ];

        if let Some(account_id) = request.account_id {
            argv.push(format!("--account-id={account_id}"));
        }

        Self { argv }
    }

    pub fn from_domain(request: DomainRequest) -> Self {
        let mut argv = vec![
            "pages".to_string(),
            "domain".to_string(),
            "add".to_string(),
            request.project_name,
            request.domain,
        ];

        if let Some(account_id) = request.account_id {
            argv.push(format!("--account-id={account_id}"));
        }

        Self { argv }
    }

    pub fn into_args(self) -> Vec<String> {
        self.argv
    }
}

#[derive(Debug, Clone)]
pub struct InvocationResult {
    status: ExitStatus,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl InvocationResult {
    pub fn from_output(output: Output) -> Self {
        Self {
            status: output.status,
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }

    pub fn is_success(&self) -> bool {
        self.status.success()
    }

    pub fn is_idempotent_failure(&self) -> bool {
        if self.status.success() {
            return false;
        }
        let text = self.output_text();
        let lowercase = text.to_lowercase();
        let phrases = [
            "already exists",
            "already added",
            "already bound",
            "already associated",
        ];
        phrases
            .iter()
            .any(|phrase| lowercase.contains(phrase))
    }

    pub fn stdout(&self) -> &[u8] {
        &self.stdout
    }

    pub fn stderr(&self) -> &[u8] {
        &self.stderr
    }

    pub fn exit_code(&self) -> i32 {
        self.status.code().unwrap_or(1)
    }

    fn output_text(&self) -> String {
        let mut combined = Vec::with_capacity(self.stdout.len() + self.stderr.len());
        combined.extend_from_slice(&self.stdout);
        combined.extend_from_slice(&self.stderr);
        String::from_utf8_lossy(&combined).to_string()
    }
}
