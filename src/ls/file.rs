pub struct File<'a> {
    pub file_str: &'a str,
}

pub struct Directory<'a> {
    pub dir_str: &'a str,
    pub file_name_vec: Vec<String>,
}

pub struct LsError {
    pub error_str: String,
}
