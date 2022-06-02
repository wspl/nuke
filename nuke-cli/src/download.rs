use anyhow::{bail, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub struct Downloader {
    client: reqwest::blocking::Client,
    url: String,
    target: PathBuf,
}

impl Downloader {
    pub fn new<P>(url: &str, target: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            client: reqwest::blocking::Client::new(),
            url: url.to_string(),
            target: target.as_ref().into(),
        }
    }

    pub fn start_download(&self) -> Result<()> {
        let mut res = self.client.get(&self.url).send()?;
        let total_size: u64 = res
            .headers()
            .get("Content-Length")
            .unwrap()
            .to_str()?
            .parse()?;
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    format!(
                        "{} [{{wide_bar:.cyan/blue}}] {{bytes}}/{{total_bytes}}",
                        "[Nuke]".green()
                    )
                    .as_str(),
                )
                .progress_chars("#>-"),
        );

        let mut file = File::create(&self.target)?;

        let mut buf = vec![0u8; 16 * 1024];
        loop {
            match res.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    file.write(&buf[0..n])?;
                    pb.inc(n as u64);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => bail!(e),
            }
        }
        pb.finish();
        Ok(())
    }
}
