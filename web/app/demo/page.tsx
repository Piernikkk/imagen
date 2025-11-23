"use client";
import { Button } from "@/components/ui/button";
import { IconPlus } from "@tabler/icons-react";
import { useState } from "react";

export default function Page() {
  const [elements, setElements] = useState([]);

  return (
    <div className="w-full h-full flex flex-col justify-center items-center gap-8">
      <h1 className="text-3xl font-bold">Generate Image</h1>
      <div className="bg-secondary p-6 md:min-w-[500] rounded-2xl min-h-[200] flex justify-center relative">
        <div className="absolute bottom-5 right-5">
          <Button size={"icon-lg"} variant={"outline"}>
            <IconPlus />
          </Button>
        </div>
        <p className="text-lg font-semibold">Elements:</p>
        <div>
          {elements.map((e, i) => (
            <div key={i}>test</div>
          ))}
        </div>
      </div>
    </div>
  );
}
