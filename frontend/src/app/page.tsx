"use client";

import React, { useState } from "react";
import { Input } from "../components/ui/input";
import { Button } from "@/components/ui/button";
import { toast } from "sonner";

export default function Home() {
  const [input, setInput] = useState("");

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className={"flex flex-col gap-4 items-center"}>
        <div className={"text-2xl"}>清华云下载器</div>

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
            onClick={() => {
              toast.info(`input: ${input}`);
            }}
          >
            解析
          </Button>
        </div>
      </div>
    </main>
  );
}
