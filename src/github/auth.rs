use crate::error::{AppError, Result};

pub struct Auth {
    token: String,
}

impl Auth {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn validate(&self) -> Result<()> {
        if self.token.is_empty() {
            return Err(AppError::Auth("Token is empty".to_string()));
        }
        if !self.token.starts_with("ghp_") && !self.token.starts_with("github_pat_") {
            return Err(AppError::Auth(
                "Invalid token format. Expected token starting with 'ghp_' or 'github_pat_'"
                    .to_string(),
            ));
        }
        Ok(())
    }
}
