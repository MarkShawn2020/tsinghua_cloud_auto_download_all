import { DirItemClient, DirItemFromServer } from "./schema";

export const dirItemServer2Client = (
  dir: DirItemFromServer,
  parentLevel: number,
): DirItemClient => ({
  is_dir: dir.is_dir,
  last_modified: dir.last_modified,
  size: dir.size,
  name: dir.is_dir ? dir.folder_name : dir.file_name,
  path: dir.is_dir ? dir.folder_path : dir.file_path,
  level: parentLevel + 1,
});
