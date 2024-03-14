"use client";

import { listen } from "@tauri-apps/api/event";
import React, { useEffect, useState } from "react";
import { DirLists } from "../components/dirs";
import { InputLine } from "../components/input";
import { DirItemClient, ListDirResult } from "../schema";
import { dirItemServer2Client } from "../utils";

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
      <div className={"flex flex-col gap-4 items-center w-full max-w-[720px]"}>
        <div className={"text-2xl"}>清华云下载器</div>

        <InputLine />

        <DirLists dirs={dirs} />
      </div>
    </main>
  );
}
