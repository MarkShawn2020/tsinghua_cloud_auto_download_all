import { atom } from "jotai";
import { atomWithImmer } from "jotai-immer";
import { atomWithStorage } from "jotai/utils";
import { number } from "prop-types";
import { DirItemClient, IServerData } from "./schema";

export const storePathAtom = atomWithStorage<string | null>("store.path", null);

export const threadsCountAtom = atomWithStorage<number>("threads.count", 4);

export const foldersVisibleAtom = atomWithStorage("folders.visible", false);

export const dirsAtom = atomWithImmer<DirItemClient[]>([]);

export const updateDirsAtom = atom(null, (get, set, data: IServerData) => {
  switch (data.type) {
    case "list-dirs":
      const { parent, children } = data;

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
      break;

    case "file-mutation":
      const { downloaded, status, filePath } = data;

      set(dirsAtom, (dirs) => {
        const dir = dirs.find((d) => d.path === filePath);
        if (!dir) return;

        dir.status = status;
        dir.downloaded = downloaded;
      });

      break;

    default:
      throw new Error("Unexpected.");
  }
});

export const fetchingAtom = atom(false);
