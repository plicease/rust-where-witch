use std::env;
use std::ffi;
use std::path;

pub enum WhereError {}

pub struct WhereIter;

impl Iterator for WhereIter {
    type Item = Result<path::PathBuf, WhereError>;
    fn next(&mut self) -> Option<Result<path::PathBuf, WhereError>> {
        None
    }
}

pub struct WhereEnv;

impl WhereEnv {
    pub fn new(
        path: Option<&ffi::OsStr>,
        include_cwd: Option<bool>,
        ext: Option<&ffi::OsStr>,
    ) -> WhereEnv {
        let path = match path {
            Some(path) => path,
            None => panic!("here"),
        };

        let path = env::split_paths(&path);

        WhereEnv
    }

    pub fn which(name: &str) -> WhereIter {
        WhereIter
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
