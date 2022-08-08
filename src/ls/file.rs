use std::path::PathBuf;

pub struct File {
    pub file_str: PathBuf,
}

pub struct Directory {
    pub dir_str: PathBuf,
    pub file_name_vec: Vec<PathBuf>,
}

pub struct LsError {
    pub error_str: String,
}
