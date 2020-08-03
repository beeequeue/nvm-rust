use std::{
    borrow::Borrow,
    fs::{create_dir_all, File},
    io,
    io::{Cursor, Error},
    path::PathBuf,
};

use clap::ArgMatches;
#[cfg(unix)]
use flate2::read::GzDecoder;
use reqwest::blocking::Response;
use semver::VersionReq;
#[cfg(unix)]
use tar::{Archive, Unpacked};
#[cfg(target_os = "windows")]
use zip::ZipArchive;

use crate::{
    node_version::{InstalledNodeVersion, NodeVersion, OnlineNodeVersion},
    subcommand::Subcommand,
    CONFIG,
};

pub struct Install;

impl Install {
    #[cfg(target_os = "windows")]
    fn extract_archive(bytes: Response, version: &OnlineNodeVersion) -> Result<(), String> {
        let version_str = version.version().to_string();
        let reader = Cursor::new(bytes.bytes().unwrap());
        let mut archive = ZipArchive::new(reader).unwrap();

        println!("Extracting...");

        for i in 0..archive.len() {
            let mut item = archive.by_index(i).unwrap();
            let file_path = item.sanitized_name();
            let file_path = file_path.to_string_lossy();

            let new_path: PathBuf = if let Some(index) = file_path.find('\\') {
                let mut path = CONFIG.dir().clone();
                path.push(version_str.clone());
                path.push(file_path[index + 1..].to_owned());

                path
            } else {
                // This happens if it's the root index, the base folder
                let mut path = CONFIG.dir().clone();
                path.push(version_str.clone());

                path
            };

            if item.is_dir() && !new_path.exists() {
                create_dir_all(new_path.to_owned())
                    .expect(format!("Could not create new folder: {:?}", new_path).borrow());
            }

            if item.is_file() {
                let mut file = File::create(&*new_path).map_err(|err| err.to_string())?;
                io::copy(&mut item, &mut file)
                    .expect(format!("Couldn't write to {:?}", new_path).borrow());
            }
        }

        Result::Ok(())
    }

    #[cfg(unix)]
    fn extract_archive(bytes: Response, version: &OnlineNodeVersion) -> Result<(), String> {
        let version_str = version.version().to_string();
        let base_path = CONFIG.dir();

        let reader = Cursor::new(bytes.bytes().unwrap());
        let tar = GzDecoder::new(reader);
        let mut archive = Archive::new(tar);

        let mut version_dir_path = base_path.clone();
        version_dir_path.push(version_str.to_owned());
        create_dir_all(version_dir_path.to_owned()).expect("fuck");

        let result = archive
            .entries()
            .map_err(|err| err.to_string())?
            .filter_map(|e| e.ok())
            .map(|mut entry| -> Result<Unpacked, Error> {
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
                entry.unpack(&new_path)
            });

        let errors: Vec<Error> = result
            .into_iter()
            .filter(|result| result.is_err())
            .map(|result| result.unwrap_err())
            .collect();

        if !errors.is_empty() {
            println!(
                "Failed to extract all files:\n{:?}",
                errors
                    .into_iter()
                    .map(|err| err.to_string())
                    .collect::<Vec<String>>()
                    .join("/n")
            );

            remove_dir_all(version_dir_path).expect("Couldn't clean up version.");
        }

        Result::Ok(())
    }

    pub fn download_and_extract_to(version: &OnlineNodeVersion) -> Result<(), String> {
        let url = version.download_url().unwrap();

        println!("Downloading from {}...", url);
        let response = reqwest::blocking::get(url)
            .map_err(|err| format!("Failed to download version: {}", err))?;

        Self::extract_archive(response, version)
    }
}

impl Subcommand for Install {
    fn run(matches: &ArgMatches) -> Result<(), String> {
        let wanted_range = VersionReq::parse(matches.value_of("version").unwrap()).unwrap();
        let force_install = matches.is_present("force");

        let online_versions = OnlineNodeVersion::fetch_all()?;
        let filtered_versions = NodeVersion::filter_version_req(online_versions, wanted_range);
        let latest_version: Option<&OnlineNodeVersion> = filtered_versions.first();

        if let Some(v) = latest_version {
            if !force_install && InstalledNodeVersion::is_installed(v.version().borrow()) {
                println!("{} is already installed - skipping...", v.version());
                return Result::Ok(());
            }

            return Install::download_and_extract_to(v.borrow());
        }

        Result::Ok(())
    }
}
