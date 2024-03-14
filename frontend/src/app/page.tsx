"use client";

import { fetchingAtom, storePathAtom, updateDirsAtom } from "@/store";
import { open } from "@tauri-apps/api/dialog";
import { listen } from "@tauri-apps/api/event";
import { useAtom } from "jotai";
import React, { useEffect } from "react";
import { toast } from "sonner";
import { DirLists } from "../components/dirs";
import { InputLine } from "../components/input";
import { Button } from "../components/ui/button";
import { Label } from "../components/ui/label";
import { IServerData } from "../schema";

export default function Home() {
  const [, updateDirs] = useAtom(updateDirsAtom);
  const [fetching] = useAtom(fetchingAtom);
  const [rootDir, setRootDir] = useAtom(storePathAtom);

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
        <div className={"text-2xl shrink-0"}>清华云下载器</div>

        <InputLine />

        <div className={"flex items-center gap-2 w-full"}>
          <Label className={"grow"}>{rootDir === null ? "空" : rootDir}</Label>

          <Button
            variant={"secondary"}
            className={"px-8"}
            size={"sm"}
            onClick={async () => {
              const selected = await open({
                directory: true,
                multiple: false,
                title: "默认存储位置",
              });

              if (Array.isArray(selected))
                return toast.error("不支持多个文件夹");
              else if (selected === null) return;

              setRootDir(selected);
            }}
          >
            存储地址
          </Button>
        </div>

        <DirLists />
      </div>
    </main>
  );
}
