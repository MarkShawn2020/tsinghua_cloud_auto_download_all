from src.config import repo, folder_path
from src.downloader.multiple import TsinghuaCloudDownloader
from src.utils import store_path

if __name__ == '__main__':
    downloader = TsinghuaCloudDownloader(store_path, repo, folder_path)
    downloader.start()
