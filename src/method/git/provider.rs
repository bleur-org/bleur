use std::path::Path;

use crate::{Error, Result};
use git2::{build::RepoBuilder, FetchOptions, Repository};
use url::Url;

pub enum Provider {
    Branch {
        provider: String,
        owner: String,
        repo: String,
        branch: String,
    },
    Branchless {
        provider: String,
        owner: String,
        repo: String,
    },
    Other {
        url: String,
    },
}

fn provider_link(scheme: String, owner: String, repo: String) -> Result<String> {
    Ok(match scheme.as_str() {
        "github" => format!("https://github.com/{owner}/{repo}"),
        // I won't support GitLab's obscure format, it's too unpredictable
        _ => return Err(Error::UnknownGitProvider(scheme.to_owned())),
    })
}

impl Provider {
    pub fn from_url(url: Url) -> Result<Self> {
        // example:owner/repo/branch
        // ---^---------------------
        if url.scheme() == "https" || url.scheme() == "http" {
            return Ok(Self::Other {
                url: url.to_string(),
            });
        }

        let path: Vec<String> = url.path().split('/').map(|i| i.to_owned()).collect();

        // example:owner/repo/branch
        // ----------^--------------
        let owner = path
            .first()
            .map(|p| p.to_owned())
            .ok_or(Error::InvalidRepositoryOwner(url.to_string()))?;

        // example:owner/repo/branch
        // ---------------^---------
        let repo = path
            .get(1)
            .map(|p| p.to_owned())
            .ok_or(Error::InvalidRepositoryName(url.to_string()))?;

        // example:owner/repo/branch
        // ---------------------^---
        let branch = path.get(2).map(|p| p.to_owned());

        Ok(match branch {
            Some(b) => Self::Branch {
                repo,
                owner,
                branch: b,
                provider: url.scheme().to_owned(),
            },
            None => Self::Branchless {
                repo,
                owner,
                provider: url.scheme().to_owned(),
            },
        })
    }

    pub fn fetch_repo(self, options: FetchOptions, path: &Path) -> Result<Repository> {
        match self {
            Self::Branch {
                provider,
                owner,
                repo,
                branch,
            } => RepoBuilder::new()
                .fetch_options(options)
                .branch(&branch)
                .clone(&provider_link(provider, owner, repo)?, path)
                .map_err(Error::GitError),
            Self::Branchless {
                provider,
                owner,
                repo,
            } => RepoBuilder::new()
                .fetch_options(options)
                .clone(&provider_link(provider, owner, repo)?, path)
                .map_err(Error::GitError),
            Self::Other { url } => RepoBuilder::new()
                .fetch_options(options)
                .clone(&url, path)
                .map_err(Error::GitError),
        }
    }
}
