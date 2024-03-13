import pathlib

from tsinghua_cloud_auto_download_all import tsinghua_cloud_auto_download_all

if __name__ == '__main__':
    cur_path = pathlib.Path(__file__)
    root_path = cur_path.parent
    store_path = root_path.joinpath('downloaded')
    store_path.mkdir(exist_ok=True)
    
    repo = '689824200edb49888695'
    tsinghua_cloud_auto_download_all(repo, store_path)
