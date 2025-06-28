use thiserror::Error;

pub type Result<T, E = BleurError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum BleurError {
    #[error("can't create temporary directory")]
    TemporaryCantCreate(#[from] std::io::Error),
    #[error("temporary directory is not created yet")]
    TemporaryNotCreated,
    #[error("can't parse this shitty url")]
    CantParseUrl(#[from] url::ParseError),
    #[error("you don't have nix nor git for initialization")]
    NoToolForInit,
    #[error("unknown data store error")]
    Unknown,
}

pub fn beautiful_exit<T>(message: T) -> !
where
    T: AsRef<str>,
{
    println!("{}", message.as_ref());
    std::process::exit(1)
}
