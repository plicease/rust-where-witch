use std::env;
use std::ffi;
use std::path;

#[cfg(unix)]
const INCLUDE_CWD_BY_DEFAULT: bool = false;

#[cfg(windows)]
const INCLUDE_CWD_BY_DEFAULT: bool = true;

pub enum WhereError {}

pub struct WhereIter;

impl Iterator for WhereIter {
    type Item = Result<path::PathBuf, WhereError>;
    fn next(&mut self) -> Option<Result<path::PathBuf, WhereError>> {
        None
    }
}

pub struct WhereConfig {
    dirs: Vec<path::PathBuf>,
    exts: Vec<ffi::OsString>,
}

impl WhereConfig {
    pub fn new(
        search_path: Option<&ffi::OsStr>,
        include_cwd: Option<bool>,
        search_ext: Option<&ffi::OsStr>,
    ) -> WhereConfig {
        let include_cwd = match include_cwd {
            Some(include_cwd) => include_cwd,
            None => INCLUDE_CWD_BY_DEFAULT,
        };

        let mut dirs: Vec<path::PathBuf> = Vec::new();

        if include_cwd {
            let cwd = env::current_dir().unwrap();
            let cwd = path::PathBuf::from(cwd.as_os_str());
            dirs.push(cwd);
        }

        match search_path {
            Some(search_path) => {
                for dir in env::split_paths(search_path) {
                    dirs.push(path::PathBuf::from(dir));
                }
            }
            None => match env::var_os("PATH") {
                Some(search_path) => {
                    for dir in env::split_paths(&search_path) {
                        let dir = dir.clone();
                        dirs.push(path::PathBuf::from(&dir));
                    }
                }
                None => (),
            },
        };

        let mut exts: Vec<ffi::OsString> = Vec::new();

        match search_ext {
            Some(search_ext) => {
                for ext in env::split_paths(search_ext) {
                    exts.push(ffi::OsString::from(ext));
                }
            }
            None => {
                #[cfg(windows)]
                match env::var_os("PATHEXT") {
                    Some(search_ext) => {
                        for ext in env::split_paths(&search_ext) {
                            exts.push(ffi::OsString::from(ext));
                        }
                    }
                    None => {
                        for ext in vec![
                            ".com", ".exe", ".bat", ".cmd", ".vbs", ".js", ".jse", ".wsf", ".wsh",
                            ".msc",
                        ]
                        .iter()
                        {
                            exts.push(ffi::OsString::from(ext));
                        }
                    }
                }
                #[cfg(unix)]
                ()
            }
        };

        WhereConfig { dirs, exts }
    }

    pub fn which(&self, name: &str) -> WhereIter {
        WhereIter
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_default_config() {
        let _ = crate::WhereConfig::new(None, None, None);
    }
}
