from typing import Union

import pydantic as pydantic


class IRepo(pydantic.BaseModel):
    repo: str
    folder_path: str


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
DONE = "DONE"
