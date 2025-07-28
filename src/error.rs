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
