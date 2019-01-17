use fscrawling;
use LogLevel;
use pathdiff::diff_paths;
use std;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};


pub type PathList = Vec<PathBuf>;


#[derive(PartialEq, Debug)]
pub struct Config {
    pub exclude_dirs: PathList,
    pub executable_file: PathBuf,
    pub log_level: LogLevel,
    pub marker_name: String,
    pub dereference_symlinks: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            exclude_dirs: vec![Path::new(".git").to_owned()],
            executable_file: PathBuf::new(),
            log_level: LogLevel::Error,
            marker_name: ".emptydir".to_string(),
            dereference_symlinks: false,
        }
    }
}


#[derive(Debug)]
pub struct Context {
    pub config: Config,
}

impl Context {
    pub fn crawl_dirs(&self, root_dirs: &PathList) -> fscrawling::DirDescriptorList {
        let crawler = fscrawling::FileSystemCrawler {
            exclude_dirs: self.config.exclude_dirs.clone(),
            dereference_symlinks: self.config.dereference_symlinks,
            marker_name: self.config.marker_name.clone(),
        };

        crawler
            .crawl_dirs(root_dirs.clone())
            .into_iter()
            .map(|(_, descr)| descr)
            .collect()
    }

    pub fn create_marker(&self, dir: &PathBuf, text: &String, dry_run: bool) -> std::io::Result<()> {
        let marker_file_path = {
            let mut dir = Context::get_absolute_dir(dir)?;
            dir.push(&self.config.marker_name);
            dir
        };

        // Write marker to disk.
        if !dry_run {
            let mut file = File::create(&marker_file_path)?;
            file.write_all(text.as_bytes())?;
        }

        info!(target: "create_marker", "Marker created: {:?}", &marker_file_path);
        Ok(())
    }

    pub fn create_marker_catched(&self, dir: &PathBuf, text: &String, dry_run: bool) {
        if let Err(error) = self.create_marker(dir, text, dry_run) {
            error!(target: "create_marker", "{}: {:?}", error, &dir);
        }
    }

    pub fn delete_child_file(&self, file: &PathBuf, dry_run: bool) -> std::io::Result<()> {
        // Remove file from disk.
        if !dry_run {
            // TODO fs::remove_file(&file)?;
        }

        info!(target: "delete_child_file", "Child file deleted: {:?}", &file);
        Ok(())
    }

    pub fn delete_child_file_catched(&self, file: &PathBuf, dry_run: bool) {
        if let Err(error) = self.delete_child_file(&file, dry_run) {
            error!(target: "delete_child_file", "{}: {:?}", error, &file);
        }
    }

    pub fn delete_child_dir(&self, dir: &PathBuf, dry_run: bool) -> std::io::Result<()> {
        // Remove dir from disk.
        if !dry_run {
            // TODO fs::remove_dir_all(&dir)?;
        }

        info!(target: "delete_child_dir", "Child dir deleted: {:?}", &dir);
        Ok(())
    }

    pub fn delete_child_dir_catched(&self, dir: &PathBuf, dry_run: bool) {
        if let Err(error) = self.delete_child_dir(&dir, dry_run) {
            error!(target: "delete_child_dir", "{}: {:?}", error, &dir);
        }
    }

    pub fn delete_marker(&self, dir: &PathBuf, dry_run: bool) -> std::io::Result<()> {
        let marker_file_path = {
            let mut dir = Context::get_absolute_dir(dir)?;
            dir.push(&self.config.marker_name);
            dir
        };

        // Remove marker from disk.
        if !dry_run {
            fs::remove_file(&marker_file_path)?;
        }

        info!(target: "delete_marker", "Marker deleted: {:?}", &marker_file_path);
        Ok(())
    }

    pub fn delete_marker_catched(&self, dir: &PathBuf, dry_run: bool) {
        if let Err(error) = self.delete_marker(&dir, dry_run) {
            error!(target: "delete_marker", "{}: {:?}", error, &dir);
        }
    }

    pub fn get_absolute_dir(dir: &PathBuf) -> std::io::Result<PathBuf> {
        if dir.is_absolute() {
            return Ok(dir.clone());
        }
        let mut abs_dir = std::env::current_dir()?;
        abs_dir.push(dir);
        Ok(abs_dir)
    }

    pub fn get_relative_dir(dir: &PathBuf, base_dir: &PathBuf) -> Option<PathBuf> {
        diff_paths(dir, base_dir)
    }

    pub fn get_relative_dir_to_current_dir(dir: &PathBuf) -> std::io::Result<Option<PathBuf>> {
        let cur_dir = std::env::current_dir()?;
        match Context::get_relative_dir(dir, &cur_dir) {
            Some(dir) => {
                let rel_dir = Path::new(".");
                if dir.iter().next().is_some() {
                    Ok(Some(rel_dir.join(dir)))
                } else {
                    Ok(Some(rel_dir.to_owned()))
                }
            }
            None => Ok(None),
        }
    }

    pub fn get_root_dir<'a>(
        &self,
        dir: &PathBuf,
        root_dirs: &'a PathList,
    ) -> std::io::Result<Option<&'a PathBuf>> {
        let dir = Context::get_absolute_dir(dir)?;
        Ok(root_dirs.iter().find(|root_dir| dir.starts_with(root_dir)))
    }
}