import { DirItemClient } from "@/schema";
import { File, Folder } from "lucide-react";
import React from "react";

export const DirItem = ({ dir }: { dir: DirItemClient }) => {
  return (
    <div className={"flex items-center gap-2 text-primary/75"}>
      {dir.is_dir ? <Folder /> : <File />}
      <span> {dir.name}</span>
    </div>
  );
};
