import pathlib
from concurrent.futures import ThreadPoolExecutor
from multiprocessing import Queue, Process

from loguru import logger

from src.core import download_file_from_tsinghua_cloud, list_files_from_tsinghua_cloud
from src.schema import DONE


class TsinghuaCloudDownloader:
    _repo: str
    _store_path: pathlib.Path
    _folder_path: str
    _q: Queue
    
    def __init__(self,
                 store_path: pathlib.Path,
                 repo: str,
                 folder_path='/',
                 ):
        self._repo = repo
        self._store_path = store_path
        self._folder_path = folder_path
        self._q = Queue()
    
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
