import React from "react";
import { DirItemClient } from "../schema";
import { DirItem } from "./dir";

export const DirLists = ({ dirs }: { dirs: DirItemClient[] }) => {
  return (
    <div className={"flex flex-col gap-2"}>
      <div>条目列表</div>

      {dirs.map((dir, index) => (
        <DirItem dir={dir} key={index} />
      ))}
    </div>
  );
};
