import { useAtomValue } from "jotai";
import React, { useEffect, useRef } from "react";
import { dirsAtom } from "../store";
import { DirItem } from "./dir";
import { sum } from "lodash";
import { filesize } from "filesize";

export const DirLists = () => {
  const dirs = useAtomValue(dirsAtom);

  const refBottom = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // refBottom.current?.scrollIntoView({ behavior: "auto" });
  }, [dirs.length]);

  const nFolders = dirs.filter((d) => d.is_dir).length;
  const nFiles = dirs.length - nFolders;
  const totalSize = filesize(sum(dirs.map((d) => d.size)));

  return (
    <div className={"w-full grow overflow-hidden flex flex-col gap-2"}>
      <div className={"shrink-0"}>
        条目列表（{nFolders}文件夹 / {nFiles}文件 / 总大小：{totalSize}）
      </div>

      <div className={"bg-muted/50 overflow-auto grow p-2 rounded-xl"}>
        {dirs.map((dir, index) => (
          <DirItem dir={dir} key={index} />
        ))}
        <div ref={refBottom} />
      </div>
    </div>
  );
};
