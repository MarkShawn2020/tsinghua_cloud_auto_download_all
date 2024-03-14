use std::io::Cursor;
use std::path::Path;

use serde_json::from_str;

use crate::schema::{DirList, ListData, TheError};

pub async fn download_file(store_path: String, repo: String, path: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("-- downloading {:?}", path);

    let response = reqwest::get(format!("https://cloud.tsinghua.edu.cn/d/{}/files/?p={}&dl=1", repo, path)).await?;

    let mut content = Cursor::new(response.bytes().await?);

    let local_path = Path::new(&store_path).join(path.clone().strip_prefix("/").unwrap());

    println!("-- local path: {:?}", local_path);

    let mut file = std::fs::File::create(local_path)?;

    std::io::copy(&mut content, &mut file)?;

    println!("File has been downloaded and saved to {:?}", path);

    Ok(())
}

pub async fn fetch_dir_list(repo: String, path: String) -> Result<DirList, TheError> {
    println!("-- listing {}", path);

    let response = reqwest::get(format!("https://cloud.tsinghua.edu.cn/api/v2.1/share-links/{}/dirents?path={}", repo, path)).await?.text().await?;

    Ok(from_str::<ListData>(&response)?.dirent_list)
}
