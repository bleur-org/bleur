use crate::{
    error::{BleurError, Result},
    method::Fetchable,
};
use reqwest::{Client, ClientBuilder};
use std::{
    fs,
    io::{self, Write},
};
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

    pub fn unarchive(&self, path: &PathBuf) -> Result<()> {
        let file = fs::File::open(path).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();

            let outpath = match file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };

            // Skip base folder, no need.
            let mut outpath = outpath.components();
            outpath.next();
            let outpath = self.path.join(outpath.as_path());

            if file.is_dir() {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = fs::File::create(&outpath).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }

            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }

        std::fs::remove_file(path).map_err(|_| BleurError::CantDeleteOldArchive)?;

        Ok(())
    }
}

impl Fetchable for Http {
    async fn fetch(&self) -> Result<()> {
        // Download the archive
        let file = self.download().await?;

        // Unarchive and then delete archive
        self.unarchive(&file)?;

        // Cooked
        Ok(())
    }
}
