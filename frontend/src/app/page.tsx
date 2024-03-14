"use client";

import { addDirsAtom, fetchingAtom } from "@/store";
import { listen } from "@tauri-apps/api/event";
import { useAtom } from "jotai";
import React, { useEffect } from "react";
import { DirLists } from "../components/dirs";
import { InputLine } from "../components/input";
import { IDirsServerData } from "../schema";

export default function Home() {
  const [, addDirs] = useAtom(addDirsAtom);
  const [fetching] = useAtom(fetchingAtom);

  useEffect(() => {
    const unListen = listen<IDirsServerData>("list_data", (event) => {
      console.log("Data from Rust: ", event.payload);

      if (!fetching) return;
      addDirs(event.payload);
    });

    return () => {
      unListen.then((fn) => fn());
    };
  }, [fetching]);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className={"flex flex-col gap-4 items-center w-full max-w-[720px]"}>
        <div className={"text-2xl"}>清华云下载器</div>

        <InputLine />

        <DirLists />
      </div>
    </main>
  );
}
