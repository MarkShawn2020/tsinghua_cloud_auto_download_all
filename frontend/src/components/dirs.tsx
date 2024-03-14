import { useAtom, useAtomValue } from "jotai";
import React, { useEffect, useRef } from "react";
import { dirsAtom, foldersVisibleAtom } from "../store";
import { DirItem } from "./dir";
import { sum } from "lodash";
import { filesize } from "filesize";

export const DirLists = () => {
  const dirs = useAtomValue(dirsAtom);
  const [foldersVisible, setFoldersVisible] = useAtom(foldersVisibleAtom);

  const refBottom = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // refBottom.current?.scrollIntoView({ behavior: "auto" });
  }, [dirs.length]);

  const files = dirs.filter((d) => !d.is_dir);
  const nFiles = files.length;
  const nFolders = dirs.length - nFiles;
  const totalSize = sum(files.map((d) => d.size));
  const downloadedSize = sum(files.map((d) => d.downloaded));

  return (
    <div className={"w-full grow overflow-hidden flex flex-col gap-2"}>
      <div className={"shrink-0"}>
        条目列表（{nFolders}文件夹 / {nFiles}文件 / 已下载：
        {filesize(downloadedSize)} / 总体积：{filesize(totalSize)} / 完成度：
        {((downloadedSize / totalSize) * 100).toFixed(2)}%）
      </div>

      <div className={"bg-muted/50 overflow-auto grow p-2 rounded-xl"}>
        {dirs
          .filter((d) => (foldersVisible ? true : !d.is_dir))
          .map((dir, index) => (
            <DirItem dir={dir} key={index} />
          ))}
        <div ref={refBottom} />
      </div>
    </div>
  );
};
