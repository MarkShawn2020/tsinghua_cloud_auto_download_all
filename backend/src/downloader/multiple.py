import pathlib
from concurrent.futures import ThreadPoolExecutor
from multiprocessing import Queue, Process

from loguru import logger

from src.core import download_file_from_tsinghua_cloud, list_files_from_tsinghua_cloud, parse_share_link
from src.schema import DONE


class TsinghuaCloudDownloader:
    _repo: str
    _store_path: pathlib.Path
    _folder_path: str
    _q: Queue
    
    def __init__(self):
        self._q = Queue()
    
    def parse_share_link(self, s: str):
        result = parse_share_link(s)
        self._repo = result.repo
        self._folder_path = result.folder_path
    
    def init_store_path(self, store_path: pathlib.Path):
        assert store_path.exists(), "路径不存在"
        assert store_path.is_dir(), "路径不是文件夹"
        self._store_path = store_path
    
    def start(self):
        put_process = Process(target=self._put_process, args=())
        get_process = Process(target=self._get_process, args=())
        
        put_process.start()
        get_process.start()
        
        # put_process.join()  # Do Not Wait
        get_process.join()
        
        logger.info("-- all processes done!")
    
    def _put_process(self):
        logger.info("-- put started")
        for item in list_files_from_tsinghua_cloud(self._store_path, self._repo, self._folder_path):
            self._q.put(item)
        self._q.put(DONE)
    
    def _get_process(self):
        logger.info("-- get started")
        with ThreadPoolExecutor() as executor:
            while True:
                item = self._q.get()
                logger.info(f'got: {item}')
                if item == DONE:
                    logger.info("-- stopped receiving new tasks")
                    break
                executor.submit(download_file_from_tsinghua_cloud, self._store_path, self._repo, item)
        logger.info('-- get done')
