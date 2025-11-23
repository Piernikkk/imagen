import { Button } from "@/components/ui/button";
import { IconBrandGithub } from "@tabler/icons-react";
import Link from "next/link";

export default function Home() {
  return (
    <div className="w-full h-full flex flex-col justify-center items-center gap-8">
      <h1 className="text-6xl font-bold">Imagen</h1>
      <p className="text-xl text-subtle">This is a demo for Imagen framework</p>

      <div className="flex gap-5">
        <Link href="/demo">
          <Button size={"xl"}>Check it out</Button>
        </Link>
        <Button size={"xl"} variant={"secondary"}>
          <IconBrandGithub />
          Github
        </Button>
      </div>
    </div>
  );
}
