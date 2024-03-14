import { invoke } from "@tauri-apps/api/tauri";
import { useAtom } from "jotai";
import React, { useEffect, useState } from "react";
import { fetchingAtom } from "../store";
import { Button } from "./ui/button";
import { Input } from "./ui/input";

export const InputLine = () => {
  const [input, setInput] = useState("");
  const [fetching, setFetching] = useAtom(fetchingAtom);

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
    <div className={"flex items-center gap-4"}>
      <Input
        value={input}
        onChange={(event) => {
          setInput(event.currentTarget.value);
        }}
      />

      <Button
        size={"sm"}
        className={"px-8"}
        variant={"secondary"}
        onClick={async () => {
          if (!fetching) {
            setFetching(true);
            await invoke<string>("fetch_data_and_emit", { path: "/" });
          } else void stopFetching();
        }}
      >
        {fetching ? "停止下载" : "开始下载"}
      </Button>
    </div>
  );
};
