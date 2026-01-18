use std::path::PathBuf;

use owo_colors::OwoColorize;
use thiserror::Error;

pub type Result<T, E = BleurError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum BleurError {
    #[error("can't create temporary directory or copy from temporary: {0}")]
    TemporaryCantCreate(#[from] std::io::Error),
    #[error("can't parse this shitty url ({0})")]
    CantParseUrl(#[from] url::ParseError),
    #[error("can't serialize given data into our type ({0})")]
    CantParseShit(#[from] serde_json::Error),
    #[error("you don't have nix nor git for initialization")]
    NoToolForInit,
    #[error("we don't have enough of arguments to decide which fetching scheme to use")]
    InsufficientArgumentsToDecide,
    #[error("failed while executing a command")]
    CommandExecutionFail,
    #[error("failed reading output of nix")]
    NixInvalidOutput(#[from] std::string::FromUtf8Error),
    #[error("something went wrong while cloning repository from remote: {0}")]
    CantCloneRepository(#[from] git2::Error),
    #[error("can't download from given url via http: {0}")]
    CantDownloadViaHttp(#[from] reqwest::Error),
    #[error("can't get length of content via http")]
    CantGetContentLength,
    #[error("can't create file to write downloads {0}")]
    CantCreateFile(String),
    #[error("can't write to file after downloading")]
    CantWriteToFile,
    #[error("can't unzip downloaded zip file: {0}")]
    CantUnArchiveZip(#[from] zip::result::ZipError),
    #[error("can't delete downloaded archive from archived directory")]
    CantDeleteOldArchive,
    #[error(
        "there seem's to be no any or valid template configuration, maybe consider creating one?"
    )]
    NoTemplateConfiguration,
    #[error("can't delete .git directory after cloning")]
    CantDeleteGitDirectorty,
    #[error("can't read/parse user prompt")]
    CantParseUserPrompt(#[from] inquire::InquireError),
    #[error("no such template in the collection")]
    NoSuchTemplateInCollection,
    #[error("during the process, bleur validated an invalid template. please, report about it at https://github.com/bleur-org/bleur/issues")]
    TemplateIsInvalid,
    #[error("path shown in template configuration seems invalid: {0}")]
    InvalidFilePath(PathBuf),
    #[error("invalid regular expression for captchuring variable names: {0}")]
    InvalidRegex(#[from] regex::Error),
    #[error("there's no such variable in bleur.toml: {0}")]
    NoSuchVariable(String),
    #[error("can't move/rename given file: {0}")]
    CantMoveFile(String),
    #[error("the given git provider is known: {0}")]
    UnknownGitProvider(String),
    #[error("the given git repository owner in the url is invalid: {0}")]
    InvalidRepositoryOwner(String),
    #[error("the given git repository name in the url is invalid: {0}")]
    InvalidRepositoryName(String),
    #[error("git error: {0}")]
    GitError(git2::Error),

    // To be used only if you get despaired.
    // Until so, don't touch, for the sake of your own sanity!
    #[error("unknown error, probably baba yaga is up to cooking something")]
    Unknown,
}

pub fn beautiful_exit<T>(message: T) -> !
where
    T: AsRef<str>,
{
    eprintln!("{} {}", "error:".red(), message.as_ref());

    std::process::exit(1)
}
