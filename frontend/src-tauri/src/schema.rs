use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DirItem {
    size: u128,
    is_dir: bool,
    last_modified: String,
    pub(crate) file_path: Option<String>,
    file_name: Option<String>,
    folder_name: Option<String>,
    pub(crate) folder_path: Option<String>,
}

pub type DirList = Vec<DirItem>;

#[derive(Serialize, Deserialize)]
pub struct ListData {
    pub(crate) dirent_list: Option<DirList>,
}
