"use client";

import { fetchingAtom, updateDirsAtom } from "@/store";
import { listen } from "@tauri-apps/api/event";
import { useAtom } from "jotai";
import React, { useEffect } from "react";
import { DirLists } from "../components/dirs";
import { InputLine } from "../components/input";
import { IServerData } from "../schema";

export default function Home() {
  const [, updateDirs] = useAtom(updateDirsAtom);
  const [fetching] = useAtom(fetchingAtom);

  useEffect(() => {
    const unListen = listen<IServerData>("core", (event) => {
      // DONT log, o.w. blocked...
      // console.log("Data from Rust: ", event.payload);

      if (!fetching) return;

      updateDirs(event.payload);
    });

    return () => {
      unListen.then((fn) => fn());
    };
  }, [fetching]);

  return (
    <main className="flex h-screen flex-col items-center justify-between p-24">
      <div
        className={
          "flex flex-col gap-4 items-center w-full max-w-[720px] h-full"
        }
      >
        <div className={"text-2xl font-bold my-8 shrink-0"}>清华云下载器</div>

        <InputLine />

        <DirLists />
      </div>
    </main>
  );
}
