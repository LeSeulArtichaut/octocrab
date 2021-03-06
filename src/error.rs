use snafu::{Backtrace, Snafu};
use std::fmt;

/// An error that could have occurred while using `Octocrab`.
#[derive(Snafu, Debug)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    GitHub {
        source: GitHubError,
        backtrace: Backtrace,
    },
    Url {
        source: url::ParseError,
        backtrace: Backtrace,
    },
    #[snafu(display("HTTP Error: {}\n\nFound at {}", source, backtrace))]
    Http {
        source: reqwest::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("JSON Error: {}\n{}\nFound at {}", source, json, backtrace))]
    Json {
        source: serde_json::Error,
        json: serde_json::Value,
        backtrace: Backtrace,
    },
    Other {
        source: Box<dyn std::error::Error + Send + Sync>,
        backtrace: Backtrace,
    },
}

/// An error returned from GitHub's API.
#[derive(serde::Deserialize, Debug, Clone)]
pub struct GitHubError {
    documentation_url: String,
    message: String,
}

impl fmt::Display for GitHubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: {}\nDocumentation URL: {}",
            self.message, self.documentation_url
        )
    }
}

impl std::error::Error for GitHubError {}
