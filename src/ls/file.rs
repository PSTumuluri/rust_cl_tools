use std::path::PathBuf;

pub struct File<'a> {
    pub file_str: &'a PathBuf,
}

pub struct Directory<'a> {
    pub dir_str: &'a PathBuf,
    pub file_name_vec: Vec<PathBuf>,
}

pub struct LsError {
    pub error_str: String,
}
