import { invoke } from "@tauri-apps/api/tauri";
import React, { useState } from "react";
import { Button } from "./ui/button";
import { Input } from "./ui/input";

export const InputLine = () => {
  const [input, setInput] = useState("");

  const trigger = async () => {
    await invoke<string>("fetch_data_and_emit", { path: "/" });
  };

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
        onClick={trigger}
      >
        解析
      </Button>
    </div>
  );
};
