"""
author: @markshawn2020
date: 2024-03-14
"""

import pathlib
from pprint import pprint
from typing import Union

import pydantic as pydantic
import requests as requests


class DirBase(pydantic.BaseModel):
    last_modified: str  # 2024-02-27T10:01:21+08:00
    size: int  # folder=0
    is_dir: bool  # file false, folder: true


class FileItem(DirBase):
    file_name: str
    file_path: str


class FolderItem(DirBase):
    folder_name: str
    folder_path: str


Dir = Union[FileItem, FolderItem]


def tsinghua_cloud_auto_download_all(
    repo: str,
    store_path=pathlib.Path(__file__).parent.joinpath('downloaded').mkdir(exist_ok=True)
):
    def download_file(item: FileItem):
        """
        todo: multiple threads
        """
        file_remote_path = f"https://cloud.tsinghua.edu.cn/d/{repo}/files/?p={item.file_path}&dl=1"
        try:
            print(f'>> downloading {file_remote_path}...')
            res = requests.get(file_remote_path)
            
            file_local_path = store_path.joinpath(item.file_path.strip('/'))
            print(f'>> dumping {file_local_path}...')
            with open(file_local_path, "wb") as f:  # dir created
                f.write(res.content)
            print(f"<< dumped {file_local_path}")
        
        except Exception as e:
            print(e)
            print(f"== failed to download {file_remote_path}")
    
    def list_items(folder_path: str):
        url = f'https://cloud.tsinghua.edu.cn/api/v2.1/share-links/{repo}/dirents?path={folder_path}'
        res = requests.get(url)
        local_dir = store_path.joinpath(folder_path.strip('/'))  # strip o.w. root
        print('-- local_dir: ', local_dir)
        local_dir.mkdir(exist_ok=True)
        
        for _item in res.json()['dirent_list']:
            pprint(_item)
            if _item['is_dir']:
                item = FolderItem.parse_obj(_item)
                list_items(item.folder_path)
            else:
                item = FileItem.parse_obj(_item)
                download_file(item)
    
    list_items('/')
