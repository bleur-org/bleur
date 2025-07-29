use crate::{
    error::{BleurError, Result},
    method::Fetchable,
};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::cmp::min;
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
            client: Client::new(),
        }
    }
}

impl Fetchable for Http {
    async fn fetch(&self) -> Result<()> {
        let res = self
            .client
            .get(self.url.clone())
            .send()
            .await
            .map_err(BleurError::CantDownloadViaHttp)?;

        let total_size = res
            .content_length()
            // .ok_or(format!("Failed to get content length from '{}'", &self.url))?;
            .ok_or(BleurError::CantGetContentLength)?;

        // Indicatif Setup
        let pb = ProgressBar::new(total_size);

        pb.set_style(ProgressStyle::default_bar()
          .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").map_err(BleurError::CantInitiateProgressTemplate)?
          .progress_chars("#>-"));
        pb.set_message(format!("Downloading {}", self.url.clone()));

        let mut file = File::create(self.path.clone())
            .map_err(|_| BleurError::CantCreateFile(self.path.to_string_lossy().to_string()))?;
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.map_err(BleurError::CantDownloadViaHttp)?;
            file.write_all(&chunk)
                .map_err(|_| BleurError::CantWriteToFile)?;
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }

        pb.finish_with_message(format!(
            "Downloaded {} to {}",
            self.url,
            self.path.to_string_lossy()
        ));

        Ok(())
    }
}
