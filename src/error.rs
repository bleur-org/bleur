use indicatif::style::TemplateError;
use owo_colors::OwoColorize;
use thiserror::Error;

pub type Result<T, E = BleurError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum BleurError {
    #[error("can't create temporary directory")]
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
    #[error("can't create the template of progressbar {0}")]
    CantInitiateProgressTemplate(#[from] TemplateError),
    #[error("can't create file to write downloads {0}")]
    CantCreateFile(String),
    #[error("can't write to file after downloading")]
    CantWriteToFile,

    // To be used only if you get despaired.
    // Until so, don't touch, for the sake of your own sanity!
    #[error("unknown error, probably baba yaga is up to cooking something")]
    Unknown,
}

pub fn beautiful_exit<T>(message: T) -> !
where
    T: AsRef<str>,
{
    eprintln!("{}: {}", "error:".red(), message.as_ref());

    std::process::exit(1)
}
