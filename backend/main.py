from src.downloader.multiple import TsinghuaCloudDownloader
from src.utils import store_path

if __name__ == '__main__':
    downloader = TsinghuaCloudDownloader()
    downloader.parse_share_link('https://cloud.tsinghua.edu.cn/d/689824200edb49888695/?p=%2F&mode=list')
    downloader.init_store_path(store_path)
    downloader.start()
