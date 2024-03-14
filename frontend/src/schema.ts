export type DirItemBase = {
  size: number;
  last_modified: string;
  is_dir: boolean;
};

export interface FolderDirItem extends DirItemBase {
  is_dir: true;
  folder_path: string;
  folder_name: string;
}

export interface FileDirItem extends DirItemBase {
  is_dir: false;
  file_name: string;
  file_path: string;
}

export type DirItemFromServer = FolderDirItem | FileDirItem;

export type FileStatus =
  | "waiting"
  | "preparing"
  | "downloading"
  | "downloaded"
  | "failed";

export interface DirItemClient extends DirItemBase {
  name: string;
  path: string;
  level: number;

  status?: FileStatus;
  downloaded?: number;
}

export type IServerData =
  | { type: "list-dirs"; children: DirItemFromServer[]; parent: string }
  | {
      type: "file-mutation";
      filePath: string;
      status: FileStatus;
      downloaded?: number;
    };
