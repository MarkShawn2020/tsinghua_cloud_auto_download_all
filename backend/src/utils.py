import pathlib

cur_path = pathlib.Path(__file__)
src_path = cur_path.parent
root_path = src_path.parent
store_path = root_path.joinpath('downloaded')
store_path.mkdir(exist_ok=True)
