import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { useAtom } from "jotai";
import { Settings } from "lucide-react";
import React, { PropsWithChildren, useEffect, useState } from "react";
import { toast } from "sonner";
import { fetchingAtom, storePathAtom, threadsCountAtom } from "../store";
import { Button } from "./ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Dialog, DialogContent, DialogTrigger } from "./ui/dialog";
import { Input } from "./ui/input";
import { Label } from "./ui/label";

export const InputLine = () => {
  const [input, setInput] = useState("");
  const [fetching, setFetching] = useAtom(fetchingAtom);
  const [storePath] = useAtom(storePathAtom);
  const [threadsCount] = useAtom(threadsCountAtom);

  const stopFetching = async () => {
    setFetching(false);
    await invoke<string>("stop_fetching");
  };

  useEffect(() => {
    window.addEventListener("beforeunload", stopFetching);

    return () => {
      void stopFetching();
      window.removeEventListener("beforeunload", stopFetching);
    };
  }, []);

  return (
    <div className={"w-full flex items-center gap-4 shrink-0"}>
      <Input
        placeholder={"https://cloud.tsinghua.edu.cn/d/xxx/?p=yyy&mode=list"}
        value={input}
        onChange={(event) => {
          setInput(event.currentTarget.value);
        }}
      />

      <Button
        className={"px-8"}
        variant={"secondary"}
        onClick={async () => {
          if (!storePath) return toast.error("请先设置好存储位置");

          if (!fetching) {
            const url = new URL(input);

            const m = url.pathname.match("/d/(.*)/");
            const repo = m ? m[1] : null;

            const rootPath = url.searchParams.get("p") || "/";
            console.log({ repo, rootPath });

            if (!repo) return toast.error("网址不合法");

            setFetching(true);
            const args = {
              repo,
              rootPath,
              storePath,
              n: threadsCount,
            };
            console.log("fetching: ", args);

            await invoke<string>("fetch_data_and_emit", args);
          } else void stopFetching();
        }}
      >
        {fetching ? "停止下载" : "确认"}
      </Button>

      <Config />
    </div>
  );
};

export const LabelLine = ({
  title,
  children,
}: { title: string } & PropsWithChildren) => {
  return (
    <div className={"flex items-center gap-2"}>
      <Label className={"shrink-0 w-16"}>{title}</Label>

      <div className={"grow"}>{children}</div>
    </div>
  );
};

export const Config = () => {
  const [rootDir, setRootDir] = useAtom(storePathAtom);
  const [threadsCount, setThreadsCount] = useAtom(threadsCountAtom);

  return (
    <Dialog>
      <DialogTrigger>
        <div
          className={"p-2 bg-muted rounded-lg hover:bg-muted/75 cursor-pointer"}
        >
          <Settings />
        </div>
      </DialogTrigger>

      <DialogContent className={"p-12"}>
        <Card>
          <CardHeader>
            <CardTitle>配置项</CardTitle>
          </CardHeader>

          <CardContent className={"space-y-8"}>
            <LabelLine title={"存储位置"}>
              <div className={"relative  w-full h-full hover:underline"}>
                {rootDir ?? "暂无"}
                <Input
                  contentEditable={false}
                  className={
                    "truncate cursor-pointer opacity-0 absolute inset-0 w-full h-full"
                  }
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
                />
              </div>
            </LabelLine>

            <LabelLine title={"线程数"}>
              <Input
                min={1}
                type={"number"}
                value={threadsCount.toString()}
                onChange={(event) => {
                  setThreadsCount(Number(event.currentTarget.value));
                }}
              />
            </LabelLine>
          </CardContent>
        </Card>
      </DialogContent>
    </Dialog>
  );
};
