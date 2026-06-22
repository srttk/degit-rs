use crate::parse::Repo;
use directories::ProjectDirs;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub struct Fetcher {
    client: Client,
    cache_dir: Option<PathBuf>,
}

impl Fetcher {
    pub fn new() -> Self {
        let cache_dir =
            ProjectDirs::from("com", "degit-rs", "degit").map(|d| d.cache_dir().to_path_buf());

        Self {
            client: Client::new(),
            cache_dir,
        }
    }
    #[allow(unused)]
    pub fn get_cache_dir(&self) -> Option<&PathBuf> {
        self.cache_dir.as_ref()
    }

    pub fn clear_cache(&self) -> std::io::Result<()> {
        if let Some(dir) = &self.cache_dir {
            if dir.exists() {
                fs::remove_dir_all(dir)?;
            }
        }
        Ok(())
    }

    pub fn fetch(&self, repo: &Repo, use_cache: bool, verbose: bool) -> Result<PathBuf, String> {
        let url = repo.download_url();
        let cache_key = format!(
            "{}-{}-{}.tar.gz",
            repo.provider_name(),
            repo.user,
            repo.name
        );

        let mut target_path = None;
        if let Some(dir) = &self.cache_dir {
            if !dir.exists() {
                fs::create_dir_all(dir).map_err(|e| e.to_string())?;
            }
            target_path = Some(dir.join(&cache_key));
        }

        if let Some(path) = &target_path {
            if use_cache && path.exists() {
                if verbose {
                    println!("Using cached tarball: {:?}", path);
                }
                return Ok(path.clone());
            }
        }

        if verbose {
            println!("Downloading {} ...", url);
        }

        let resp = self
            .client
            .get(&url)
            .header(
                USER_AGENT,
                "degit-rs (https://github.com/Rich-Harris/degit)",
            )
            .send()
            .map_err(|e| format!("Failed to download {}: {}", url, e))?;

        if !resp.status().is_success() {
            return Err(format!(
                "Could not download {}: status {}",
                url,
                resp.status()
            ));
        }

        let bytes = resp.bytes().map_err(|e| e.to_string())?;

        if let Some(path) = &target_path {
            let mut file =
                File::create(path).map_err(|e| format!("Could not create cache file: {}", e))?;
            file.write_all(&bytes)
                .map_err(|e| format!("Could not write cache file: {}", e))?;
            Ok(path.clone())
        } else {
            let temp_path = PathBuf::from(&cache_key);
            let mut file = File::create(&temp_path).map_err(|e| e.to_string())?;
            file.write_all(&bytes).map_err(|e| e.to_string())?;
            Ok(temp_path)
        }
    }
}
