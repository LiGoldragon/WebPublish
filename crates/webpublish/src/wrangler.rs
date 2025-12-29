use std::io;
use std::process::Command;

use crate::invocation::{InvocationResult, WranglerArgv};

#[derive(Debug, Clone)]
pub struct Wrangler {
    path: String,
}

impl Wrangler {
    pub fn from_path() -> Self {
        Self {
            path: "wrangler".to_string(),
        }
    }

    pub fn run(&self, argv: WranglerArgv) -> io::Result<InvocationResult> {
        let output = Command::new(&self.path).args(argv.into_args()).output()?;
        Ok(InvocationResult::from_output(output))
    }
}
