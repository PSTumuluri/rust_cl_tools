use std::fs::Metadata;
use std::path::Path;

pub fn make_long_list_string(path: &Path) -> String {
    let mut result = String::new();

    let metadata = path.metadata()
        .expect(format!("Metadata not found for file {}", path.display()).as_str());

    result.push_str(&make_file_mode_string(&metadata));

    result
}

fn make_file_mode_string(metadata: &Metadata) -> String {
}
