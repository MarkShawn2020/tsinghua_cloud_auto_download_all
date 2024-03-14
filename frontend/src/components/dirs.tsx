import { useAtomValue } from "jotai";
import React from "react";
import { dirsAtom } from "../store";
import { DirItem } from "./dir";

export const DirLists = () => {
  const dirs = useAtomValue(dirsAtom);

  return (
    <div className={"w-full flex flex-col gap-2"}>
      <div>条目列表</div>

      {dirs.map((dir, index) => (
        <DirItem dir={dir} key={index} />
      ))}
    </div>
  );
};
