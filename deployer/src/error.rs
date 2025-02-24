use std::error::Error as StdError;
use std::io;
use thiserror::Error;

use shuttle_service::loader::LoaderError;

use cargo::util::errors::CargoTestError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Internal I/O error: {0}")]
    InputOutput(#[from] io::Error),
    #[error("Build error: {0}")]
    Build(#[source] Box<dyn StdError + Send>),
    #[error("Prepare to load error: {0}")]
    PrepareLoad(String),
    #[error("Load error: {0}")]
    Load(#[from] LoaderError),
    #[error("Run error: {0}")]
    Run(#[from] shuttle_service::Error),
    #[error("Pre-deployment test failure: {0}")]
    PreDeployTestFailure(#[from] TestError),
    #[error("Failed to parse secrets: {0}")]
    SecretsParse(#[from] toml::de::Error),
    #[error("Failed to set secrets: {0}")]
    SecretsSet(#[source] Box<dyn StdError + Send>),
    #[error("Failed to cleanup old deployments: {0}")]
    OldCleanup(#[source] Box<dyn StdError + Send>),
}

#[derive(Error, Debug)]
pub enum TestError {
    #[error("Tests failed: {0}")]
    Failed(#[from] CargoTestError),
    #[error("Failed to setup tests run: {0}")]
    Setup(#[from] anyhow::Error),
    #[error("Failed to run tests: {0}")]
    Run(#[from] tokio::task::JoinError),
}

pub type Result<T> = std::result::Result<T, Error>;
