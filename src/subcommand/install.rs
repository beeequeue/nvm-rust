#[cfg(unix)]
use std::fs::remove_dir_all;
#[cfg(windows)]
use std::fs::File;
#[cfg(windows)]
use std::io::copy;
use std::{borrow::Borrow, fs::create_dir_all, io::Cursor, path::PathBuf};

use anyhow::{Context, Result};
use clap::ArgMatches;
#[cfg(unix)]
use flate2::read::GzDecoder;
use reqwest::blocking::Response;
use semver::{Compat, VersionReq};
#[cfg(unix)]
use tar::{Archive, Unpacked};
#[cfg(target_os = "windows")]
use zip::ZipArchive;

use crate::{
    config::Config,
    node_version::{InstalledNodeVersion, NodeVersion, OnlineNodeVersion},
    subcommand::Subcommand,
};

pub struct Install<'c> {
    config: &'c Config,
}

impl<'c> Install<'c> {
    #[cfg(target_os = "windows")]
    fn extract_archive(self, bytes: Response, version: &OnlineNodeVersion) -> Result<()> {
        let version_str = version.version().to_string();
        let reader = Cursor::new(bytes.bytes().unwrap());
        let mut archive = ZipArchive::new(reader).unwrap();

        println!("Extracting...");

        for i in 0..archive.len() {
            let mut item = archive.by_index(i).unwrap();
            let file_path = item.sanitized_name();
            let file_path = file_path.to_string_lossy();

            let new_path: PathBuf = if let Some(index) = file_path.find('\\') {
                let mut path = self.config.dir.to_owned();
                path.push(version_str.clone());
                path.push(file_path[index + 1..].to_owned());

                path
            } else {
                // This happens if it's the root index, the base folder
                let mut path = self.config.dir.to_owned();
                path.push(version_str.clone());

                path
            };

            if item.is_dir() && !new_path.exists() {
                create_dir_all(new_path.to_owned()).unwrap_or_else(|_| {
                    panic!("Could not create new folder: {:?}", new_path)
                });
            }

            if item.is_file() {
                let mut file = File::create(&*new_path)?;
                copy(&mut item, &mut file)
                    .unwrap_or_else(|_| panic!("Couldn't write to {:?}", new_path));
            }
        }

        let extraction_dir = self.config.dir.join(version_str);
        let extraction_dir = extraction_dir.to_str().unwrap();
        println!(
            "Extracted to {}",
            // Have to remove \\?\ prefix 🤮
            extraction_dir
                .strip_prefix("\\\\?\\")
                .unwrap_or(extraction_dir)
        );

        Result::Ok(())
    }

    #[cfg(unix)]
    fn extract_archive(self, bytes: Response, version: &OnlineNodeVersion) -> Result<()> {
        let version_str = version.version().to_string();
        let base_path = self.config.dir.to_owned();

        let reader = Cursor::new(bytes.bytes().unwrap());
        let tar = GzDecoder::new(reader);
        let mut archive = Archive::new(tar);

        let mut version_dir_path = base_path.clone();
        version_dir_path.push(version_str.to_owned());
        create_dir_all(version_dir_path.to_owned()).expect("fuck");

        println!("Extracting...");

        let result = archive
            .entries()
            .map_err(anyhow::Error::from)?
            .filter_map(|e| e.ok())
            .map(|mut entry| -> Result<Unpacked> {
                let file_path = entry.path()?.to_owned();
                let file_path = file_path.to_str().unwrap();

                let new_path: PathBuf = if let Some(index) = file_path.find('/') {
                    let mut path = base_path.clone();
                    path.push(version_str.clone());
                    path.push(file_path[index + 1..].to_owned());

                    path
                } else {
                    // This happens if it's the root index, the base folder
                    let mut path = base_path.clone();
                    path.push(version_str.clone());

                    path
                };

                entry.set_preserve_permissions(false);
                entry.unpack(&new_path).map_err(anyhow::Error::from)
            });

        let errors: Vec<anyhow::Error> = result
            .into_iter()
            .filter(|result| result.is_err())
            .map(|result| result.unwrap_err())
            .collect();

        if !errors.is_empty() {
            remove_dir_all(version_dir_path).expect("Couldn't clean up version.");

            return Result::Err(anyhow::anyhow!(
                "Failed to extract all files:\n{:?}",
                errors
                    .into_iter()
                    .map(|err| err.to_string())
                    .collect::<Vec<String>>()
                    .join("/n")
            ));
        }

        println!("Extracted to {:?}", version_dir_path);

        Result::Ok(())
    }

    pub fn download_and_extract_to(self, version: &OnlineNodeVersion) -> Result<()> {
        let url = version.get_download_url().unwrap();

        println!("Downloading from {}...", url);
        let response = reqwest::blocking::get(url)
            .context(format!("Failed to download version: {}", version.version()))?;

        self.extract_archive(response, version)
    }
}

impl<'c> Subcommand<'c> for Install<'c> {
    fn run(config: &'c Config, matches: &ArgMatches) -> Result<()> {
        let command = Self { config };

        let input = matches.value_of("version").unwrap();
        let wanted_range = VersionReq::parse_compat(input, Compat::Npm).unwrap();
        let force_install = matches.is_present("force");

        let online_versions = OnlineNodeVersion::fetch_all()?;
        let filtered_versions = <dyn NodeVersion>::filter_version_req(online_versions, &wanted_range);
        let latest_version: Option<&OnlineNodeVersion> = filtered_versions.first();

        if let Some(v) = latest_version {
            if !force_install && InstalledNodeVersion::is_installed(config, v.version().borrow()) {
                println!("{} is already installed - skipping...", v.version());
                return Result::Ok(());
            }

            command.download_and_extract_to(v.borrow())
        } else {
            anyhow::bail!(
                "Did not find a version matching `{}`, (parsed as `{}`)",
                input,
                wanted_range
            )
        }
    }
}
