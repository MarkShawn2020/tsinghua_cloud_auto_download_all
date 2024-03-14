import { atom } from "jotai";
import { atomWithImmer } from "jotai-immer";
import { atomWithStorage } from "jotai/utils";
import { DirItemClient, IDirsServerData } from "./schema";

export const storePathAtom = atomWithStorage<string | null>("store.path", null);

export const dirsAtom = atomWithImmer<DirItemClient[]>([]);

export const addDirsAtom = atom(null, (get, set, data: IDirsServerData) => {
  const { children, parent } = data;

  set(dirsAtom, (dirs) => {
    const index = dirs.findIndex((d) => d.path === parent);
    const newLevel = index >= 0 ? dirs[index].level + 1 : 0;

    console.log({ dirs, parent, index });

    const newData = children.map((c) => ({
      ...c,
      path: c.is_dir ? c.folder_path : c.file_path,
      name: c.is_dir ? c.folder_name : c.file_name,
      level: newLevel,
    }));

    dirs.splice(index, 0, ...newData);
  });
});

export const fetchingAtom = atom(false);
