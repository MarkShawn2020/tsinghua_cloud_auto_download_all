"use client";

import { Button } from "@/components/ui/button";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { File, Folder } from "lucide-react";
import React, { useEffect, useState } from "react";
import { Input } from "../components/ui/input";

type DirItemBase = {
  size: number;
  last_modified: string;
  is_dir: boolean;
};

interface FolderDirItem extends DirItemBase {
  is_dir: true;
  folder_path: string;
  folder_name: string;
}

interface FileDirItem extends DirItemBase {
  is_dir: false;
  file_name: string;
  file_path: string;
}

type DirItemFromServer = FolderDirItem | FileDirItem;

type ListDirResult = {
  dirent_list?: DirItemFromServer[];
};

interface DirItemClient extends DirItemBase {
  name: string;
  path: string;
}

const dirItemServer2Client = (dir: DirItemFromServer): DirItemClient => ({
  is_dir: dir.is_dir,
  last_modified: dir.last_modified,
  size: dir.size,
  name: dir.is_dir ? dir.folder_name : dir.file_name,
  path: dir.is_dir ? dir.folder_path : dir.file_path,
});

export default function Home() {
  const [dirs, setDirs] = useState<DirItemClient[]>([]);

  useEffect(() => {
    const unListen = listen<{ data: ListDirResult }>("list_data", (event) => {
      console.log("Data from Rust: ", event.payload);

      const data = event.payload.data;
      console.log("Data parsed: ", data);
      if (data.dirent_list) setDirs(data.dirent_list.map(dirItemServer2Client));
    });

    return () => {
      unListen.then((fn) => fn());
    };
  }, []);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className={"flex flex-col gap-4 items-center"}>
        <div className={"text-2xl"}>清华云下载器</div>

        <InputLine />

        <DirLists dirs={dirs} />
      </div>
    </main>
  );
}

const InputLine = () => {
  const [input, setInput] = useState("");

  const trigger = async () => {
    await invoke<string>("fetch_data_and_emit", { path: "/" });
  };

  return (
    <div className={"flex items-center gap-4"}>
      <Input
        value={input}
        onChange={(event) => {
          setInput(event.currentTarget.value);
        }}
      />
      <Button
        size={"sm"}
        className={"px-8"}
        variant={"secondary"}
        onClick={trigger}
      >
        解析
      </Button>
    </div>
  );
};

const DirLists = ({ dirs }: { dirs: DirItemClient[] }) => {
  return (
    <div className={"flex flex-col gap-2"}>
      <div>条目列表</div>

      {dirs.map((dir, index) => (
        <DirItem dir={dir} key={index} />
      ))}
    </div>
  );
};

const DirItem = ({ dir }: { dir: DirItemClient }) => {
  return (
    <div className={"flex items-center gap-2 text-primary/75"}>
      {dir.is_dir ? <Folder /> : <File />}
      <span> {dir.name}</span>
    </div>
  );
};
