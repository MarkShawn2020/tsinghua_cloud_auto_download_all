use serde_json::from_str;
use crate::schema::{DirList, ListData, TheError};

pub async fn fetch_data(path: &str) -> Result<DirList, TheError> {
    println!("fetching data...");

    let response = reqwest::get(format!("https://cloud.tsinghua.edu.cn/api/v2.1/share-links/689824200edb49888695/dirents?path={}", path))
        .await?
        .text()
        .await?;

    Ok(from_str::<ListData>(&response)?.dirent_list)
}
