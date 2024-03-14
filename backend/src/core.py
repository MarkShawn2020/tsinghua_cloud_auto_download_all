import json
from pathlib import Path

import requests
from loguru import logger

from src.schema import FolderItem, FileItem


def list_files_from_tsinghua_cloud(store_path: Path, repo: str, folder_path: str):
    """
    yield files
    :param store_path:
    :param repo:
    :param folder_path:
    :return:
    """
    url = f'https://cloud.tsinghua.edu.cn/api/v2.1/share-links/{repo}/dirents?path={folder_path}'
    res = requests.get(url)
    local_dir = store_path.joinpath(folder_path.strip('/'))  # strip o.w. root
    logger.info(f'local_dir: {local_dir}')
    local_dir.mkdir(exist_ok=True)
    
    for _item in res.json().get('dirent_list', []):
        
        # logger.debug(json.dumps(_item, indent=2, ensure_ascii=False))
        
        if _item['is_dir']:
            item = FolderItem.parse_obj(_item)
            yield from list_files_from_tsinghua_cloud(store_path, repo, item.folder_path)
        else:
            item = FileItem.parse_obj(_item)
            logger.info(f'  put: {item.file_path}')
            yield item.file_path


def download_file_from_tsinghua_cloud(store_path: Path, repo: str, file_path: str):
    file_remote_path = f"https://cloud.tsinghua.edu.cn/d/{repo}/files/?p={file_path}&dl=1"
    try:
        logger.info(f'>> downloading {file_remote_path}...')
        res = requests.get(file_remote_path)
        
        file_local_path = store_path.joinpath(file_path.strip('/'))
        logger.info(f'>> dumping {file_local_path}...')
        with open(file_local_path, "wb") as f:  # dir created
            f.write(res.content)
        logger.info(f"<< dumped {file_local_path}")
    
    except Exception as e:
        logger.warning(e)
        logger.warning(f"== failed to download {file_remote_path}")
