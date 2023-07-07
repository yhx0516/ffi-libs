use std::{
    io,
    path::{Path, PathBuf},
};

pub fn canonicalize_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    #[cfg(target_os = "windows")]
    let path = dunce::canonicalize(path.as_ref());

    #[cfg(not(target_os = "windows"))]
    let path = path.as_ref().canonicalize();

    path
}

pub fn norm_path(path: impl AsRef<Path>) -> String {
    let str = path.as_ref().display().to_string();
    str.replace("\\", "/").trim_end_matches('/').to_string()
}

pub fn norm_path_extreme(path: impl AsRef<Path>) -> String {
    let str = path.as_ref().display().to_string();
    str.replace("\\", "/").trim_matches('/').to_string()
}
