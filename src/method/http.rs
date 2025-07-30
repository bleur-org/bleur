use crate::{
    error::{BleurError, Result},
    method::Fetchable,
};

use reqwest::{Client, ClientBuilder};
use std::io::Write;
use std::{fs::File, path::PathBuf};
use tokio_stream::StreamExt;
use url::Url;

#[derive(Debug)]
pub struct Http {
    url: Url,
    path: PathBuf,
    client: Client,
}

impl Http {
    pub fn new(url: Url, path: PathBuf) -> Self {
        Self {
            url,
            path,
            client: ClientBuilder::new().build().unwrap_or_default(),
        }
    }

    pub async fn download(&self) -> Result<PathBuf> {
        let res = self
            .client
            .get(self.url.clone())
            .send()
            .await
            .map_err(BleurError::CantDownloadViaHttp)?;

        let path = self
            .path
            .join(self.url.path().split('/').next_back().unwrap());

        let mut file = File::create(&path)
            .map_err(|_| BleurError::CantCreateFile(self.path.to_string_lossy().to_string()))?;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.map_err(BleurError::CantDownloadViaHttp)?;
            file.write_all(&chunk)
                .map_err(|_| BleurError::CantWriteToFile)?;
        }

        Ok(path)
    }

    pub fn unarchive() -> Result<()> {
        Ok(())
    }
}

impl Fetchable for Http {
    async fn fetch(&self) -> Result<()> {
        let file = self.download().await?;

        Ok(())
    }
}
