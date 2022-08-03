use std::fs::{Metadata, FileType};
use std::path::Path;

pub fn make_long_list_string(path: &Path) -> String {
    let mut result = String::new();

    let metadata = path.metadata()
        .expect(format!("Metadata not found for file {}", path.display()).as_str());

    result.push_str(make_file_mode_string(&metadata).as_str());

    result.push('\t');

    result.push_str(path.file_name().unwrap().to_str().unwrap());

    result
}

fn make_file_mode_string(metadata: &Metadata) -> String {
    let mut result = String::new();

    let file_type = metadata.file_type();
    push_file_type(&mut result, &file_type);

    // let permissions = metadata.permissions();
    // push_user_permissions(&mut result, &permissions);
 
    result
}

fn push_file_type(s: &mut String, file_type: &FileType) {
    if file_type.is_file() {
        s.push('-');
    } else if file_type.is_dir() {
        s.push('d');
    } else if file_type.is_symlink() {
        s.push('l');
    }
}
