#[derive(Debug)]
pub struct VersionInfo {
    pub major: u16,
    pub minor: u16,
    pub bugfix: u16,
    pub suffix: Option<&'static str>,
}

impl VersionInfo {
    pub fn new(major: u16, minor: u16, bugfix: u16) -> VersionInfo {
        VersionInfo {
            major: major,
            minor: minor,
            bugfix: bugfix,
            suffix: None,
        }
    }

    pub fn new_with_suffix(
        major: u16,
        minor: u16,
        bugfix: u16,
        suffix: &'static str,
    ) -> VersionInfo {
        VersionInfo {
            major: major,
            minor: minor,
            bugfix: bugfix,
            suffix: Some(suffix),
        }
    }
}

impl std::fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)?;
        if self.bugfix > 0 {
            write!(f, ".{}", self.bugfix)?;
        }
        if let Some(ref suffix) = self.suffix {
            write!(f, "-{}", &suffix)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ApplicationInfo {
    pub copyright_year: u16,
    pub license: &'static str,
    pub name: &'static str,
    pub site: &'static str,
    pub vendor_email: &'static str,
    pub vendor_name: &'static str,
    pub version_info: VersionInfo,
}
