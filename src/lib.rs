use std::env;
use std::ffi;
use std::path;

#[cfg(unix)]
const INCLUDE_CWD_BY_DEFAULT: bool = false;

#[cfg(unix)]
const PATH_SEP: &str = ":";

#[cfg(windows)]
const INCLUDE_CWD_BY_DEFAULT: bool = false;

#[cfg(windows)]
const PATH_SEP: &str = ";";

#[cfg(unix)]
const PATHEXT_DEFAULT: &str = "";

/* This is the default for Windows Vista and later...
 * Windows 7 is done in a couple of weeks as of this writing
 * so we won't support it
 */
#[cfg(windows)]
const PATHEXT_DEFAULT: &str = ".com;.exe;.bat;.cmd;.vbs;.vbe;.js;.jse;.wsf;.wsh;.msc";

pub enum WhereError {}

pub struct WhereIter;

impl Iterator for WhereIter {
    type Item = Result<path::PathBuf, WhereError>;
    fn next(&mut self) -> Option<Result<path::PathBuf, WhereError>> {
        None
    }
}

pub struct WhereConfig {
    path: ffi::OsString,
    ext: ffi::OsString,
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

        let mut path = ffi::OsString::new();

        if include_cwd {
            path.push(env::current_dir().unwrap().as_os_str());
        }

        match search_path {
            Some(search_path) => {
                if include_cwd {
                    path.push(PATH_SEP);
                }
                path.push(search_path);
            }
            None => match env::var_os("PATH") {
                Some(search_path) => {
                    if include_cwd {
                        path.push(PATH_SEP);
                    }
                    path.push(search_path);
                }
                None => (),
            },
        };

        let ext = match search_ext {
            Some(search_ext) => ffi::OsString::from(search_ext),
            None => ffi::OsString::from(PATHEXT_DEFAULT),
        };

        WhereConfig {
            path: path,
            ext: ext,
        }
    }

    pub fn which(name: &str) -> WhereIter {
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
