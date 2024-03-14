import { DirItemClient, FileStatus } from "@/schema";
import { foldersVisibleAtom } from "@/store";
import { useAtom } from "jotai";
import { File, Folder } from "lucide-react";
import React from "react";
import { Slider } from "./ui/slider";
import { Badge } from "./ui/badge";

export const DirItem = ({ dir }: { dir: DirItemClient }) => {
  const [foldersVisible, setFoldersVisible] = useAtom(foldersVisibleAtom);
  const percentage = ((dir.downloaded ?? 0) / dir.size) * 100;

  return (
    <div
      className={"flex items-center gap-2 text-primary/75"}
      style={{
        marginLeft: foldersVisible ? `${dir.level * 16}px` : `0px`,
      }}
    >
      <div className={"flex items-center gap-2 shrink-0 w-1/2"}>
        <span className={"shrink-0"}>{dir.is_dir ? <Folder /> : <File />}</span>

        <span className={"truncate"}> {dir.name}</span>
      </div>

      <div className={"shrink-0"}>
        <DownloadingStatus status={dir.status} />
      </div>

      <Slider min={0} max={100} value={[percentage]} />

      <span className={"ml-auto"}>{percentage.toFixed(2)}%</span>
    </div>
  );
};

export const DownloadingStatus = ({ status }: { status?: FileStatus }) => {
  switch (status) {
    case "downloaded":
      return (
        <Badge className={"w-20 justify-center bg-green-500"}>已下载</Badge>
      );

    case "downloading":
      return (
        <Badge className={"w-20 justify-center bg-blue-500"}>正在下载</Badge>
      );

    case "failed":
      return <Badge variant={"destructive"}>失败</Badge>;

    case "waiting":
      return (
        <Badge className={"w-20 justify-center bg-gray-500"}>准备中</Badge>
      );

    case "preparing":
      return (
        <Badge className={"w-20 justify-center bg-cyan-800"}>正在开始</Badge>
      );

    default:
      return;
  }
};
