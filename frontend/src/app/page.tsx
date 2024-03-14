"use client";

import { Button } from "@/components/ui/button";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import React, { useEffect, useState } from "react";
import { Input } from "../components/ui/input";

type DirItem = {
  size: number;
  last_modified: string;
} & (
  | {
      is_dir: true;
      folder_path: string;
      folder_name: string;
    }
  | {
      is_dir: false;
      file_name: string;
      file_path: string;
    }
);

type ListDirResult = {
  dirent_list?: DirItem[];
};

export default function Home() {
  const [dirs, setDirs] = useState<DirItem[]>([]);

  useEffect(() => {
    const unListen = listen<{ data: string }>("list_data", (event) => {
      console.log("Data from Rust: ", event.payload);

      const data = JSON.parse(event.payload.data) as ListDirResult;
      console.log("Data parsed: ", data);
      if (data.dirent_list) setDirs(data.dirent_list);
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
    await invoke<string>("fetch_data_and_emit");
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

const DirLists = ({ dirs }: { dirs: DirItem[] }) => {
  return (
    <div className={"flex flex-col gap-2"}>
      <div>条目列表</div>

      {dirs.map((dir, index) => (
        <div key={index}>
          <span>isDir: {dir.is_dir ? "TRUE" : "FALSE"}</span>
          <span> {dir.is_dir ? dir.folder_path : dir.file_path}</span>
        </div>
      ))}
    </div>
  );
};
