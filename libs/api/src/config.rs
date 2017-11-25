use log::LogLevel;
use std::fmt;
use std::path::{Path, PathBuf};


pub type PathList = Vec<PathBuf>;


#[derive(Debug)]
pub struct VersionInfo {
    pub major: u16,
    pub minor: u16,
    pub bugfix: u16,
    pub suffix: String,
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)?;
        if self.bugfix > 0 {
            write!(f, ".{}", self.bugfix)?;
        }
        if !self.suffix.is_empty() {
            write!(f, "-{}", &self.suffix)?;
        }
        Ok(())
    }
}


#[derive(Debug)]
pub struct ApplicationInfo {
    pub copyright: String,
    pub disclaimer: String,
    pub license: String,
    pub name: String,
    pub site: String,
    pub vendor_email: String,
    pub vendor_name: String,
    pub version_info: VersionInfo,
}


#[derive(PartialEq, Debug)]
pub struct Config {
    // pub application_info: ApplicationInfo,
    pub exclude_dirs: PathList,
    pub executable_file: String,
    pub log_level: LogLevel,
    pub marker_name: String,
    pub dereference_symlinks: bool,
}

impl Config {
    pub fn new() -> Config {
        // pub fn new(application_info: ApplicationInfo) -> Config {
        Config {
            // application_info: application_info,
            exclude_dirs: vec![Path::new(".git").to_owned()],
            executable_file: "".to_string(),
            log_level: LogLevel::Error,
            marker_name: ".emptydir".to_string(),
            dereference_symlinks: false,
        }
    }
}


#[derive(PartialEq, Debug)]
pub enum CommandListFilter {
    Clashing,
    Correct,
    Missing,
}

#[derive(PartialEq, Debug)]
pub enum Command {
    Clean {
        delete_hook: String,
        dry_run: bool,
        root_dirs: PathList,
    },
    List {
        filter: Vec<CommandListFilter>,
        root_dirs: PathList,
    },
    Purge { dry_run: bool, root_dirs: PathList },
    Update {
        create_hook: String,
        delete_hook: String,
        dry_run: bool,
        marker_text: String,
        root_dirs: PathList,
        substitute_variables: bool,
    },
}
