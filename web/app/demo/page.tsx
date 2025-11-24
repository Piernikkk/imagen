"use client";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Spinner } from "@/components/ui/spinner";
import { useDialogs } from "@/lib/DialogManager";
import { $api } from "@/lib/providers/api";
import { Commands } from "@/lib/providers/types";
import { IconPlus, IconPhoto } from "@tabler/icons-react";
import { useState } from "react";
import Tile from "@/components/Tile";

interface Color {
  r: number;
  g: number;
  b: number;
  a: number;
}

export default function Page() {
  const dialogs = useDialogs();

  const [elements, setElements] = useState<Commands[]>([]);
  const [canvasWidth, setCanvasWidth] = useState<number>(800);
  const [canvasHeight, setCanvasHeight] = useState<number>(600);
  const [background, setBackground] = useState<Color>({
    r: 255,
    g: 255,
    b: 255,
    a: 255,
  });
  const [isGenerating, setIsGenerating] = useState(false);

  const build = $api.useMutation("post", "/api/build");

  async function handleAddElement() {
    const command = await dialogs.show("Command");
    setElements((e) => [...e, command as Commands]);
  }

  function handleRemoveElement(index: number) {
    setElements((e) => e.filter((_, i) => i !== index));
  }

  function handleBackgroundChange(channel: keyof Color, value: string) {
    const numValue = Math.min(255, Math.max(0, parseInt(value) || 0));
    setBackground((prev) => ({
      ...prev,
      [channel]: numValue,
    }));
  }

  async function handleGenerateImage() {
    setIsGenerating(true);
    try {
      const response = await fetch("/api/build", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          width: canvasWidth,
          height: canvasHeight,
          background: background,
          commands: elements,
        }),
      });

      if (!response.ok) {
        throw new Error(`Failed to generate image: ${response.statusText}`);
      }

      const blob = await response.blob();
      const imageUrl = URL.createObjectURL(blob);
      const link = document.createElement("a");
      link.href = imageUrl;
      link.download = `imagen-${Date.now()}.png`;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      URL.revokeObjectURL(imageUrl);
    } catch (error) {
      console.error("Failed to generate image:", error);
    } finally {
      setIsGenerating(false);
    }
  }

  return (
    <div className="w-full h-full flex flex-col justify-center items-center gap-8 p-8">
      <h1 className="text-3xl font-bold">Generate Image</h1>

      <div className="bg-secondary p-6 rounded-2xl w-full max-w-2xl">
        <div className="text-lg font-semibold mb-4">Canvas Settings</div>
        <div className="grid grid-cols-2 gap-4 mb-4">
          <div className="space-y-2">
            <Label htmlFor="width">Width (px)</Label>
            <Input
              id="width"
              type="number"
              min="1"
              max="4096"
              value={canvasWidth}
              onChange={(e) => setCanvasWidth(parseInt(e.target.value) || 800)}
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="height">Height (px)</Label>
            <Input
              id="height"
              type="number"
              min="1"
              max="4096"
              value={canvasHeight}
              onChange={(e) => setCanvasHeight(parseInt(e.target.value) || 600)}
            />
          </div>
        </div>

        <div className="space-y-2">
          <Label>Background Color</Label>
          <div className="flex gap-2 items-center">
            <div className="grid grid-cols-4 gap-2 flex-1">
              <div>
                <Label htmlFor="bg-r" className="text-xs">
                  R
                </Label>
                <Input
                  id="bg-r"
                  type="number"
                  min="0"
                  max="255"
                  value={background.r}
                  onChange={(e) => handleBackgroundChange("r", e.target.value)}
                />
              </div>
              <div>
                <Label htmlFor="bg-g" className="text-xs">
                  G
                </Label>
                <Input
                  id="bg-g"
                  type="number"
                  min="0"
                  max="255"
                  value={background.g}
                  onChange={(e) => handleBackgroundChange("g", e.target.value)}
                />
              </div>
              <div>
                <Label htmlFor="bg-b" className="text-xs">
                  B
                </Label>
                <Input
                  id="bg-b"
                  type="number"
                  min="0"
                  max="255"
                  value={background.b}
                  onChange={(e) => handleBackgroundChange("b", e.target.value)}
                />
              </div>
              <div>
                <Label htmlFor="bg-a" className="text-xs">
                  A
                </Label>
                <Input
                  id="bg-a"
                  type="number"
                  min="0"
                  max="255"
                  value={background.a}
                  onChange={(e) => handleBackgroundChange("a", e.target.value)}
                />
              </div>
            </div>
            <div
              className="w-16 h-16 rounded border border-border shrink-0"
              style={{
                backgroundColor: `rgba(${background.r}, ${background.g}, ${background.b}, ${background.a / 255})`,
              }}
            />
          </div>
        </div>
      </div>

      <div className="bg-secondary p-6 rounded-2xl w-full max-w-2xl min-h-[200px] flex flex-col relative">
        <div className="absolute bottom-5 right-5">
          <Button
            size={"icon-lg"}
            variant={"outline"}
            onClick={handleAddElement}>
            <IconPlus />
          </Button>
        </div>
        <div className="text-lg font-semibold mb-4">Elements:</div>
        <div className="flex flex-col gap-2 w-full px-4 pb-16">
          {elements.map((command, i) => (
            <Tile
              key={i}
              command={command}
              onRemove={() => handleRemoveElement(i)}
            />
          ))}
          {elements.length === 0 && (
            <div className="text-center text-muted-foreground py-8">
              No commands added yet. Click + to add one.
            </div>
          )}
        </div>
      </div>

      <Button
        size="lg"
        onClick={handleGenerateImage}
        disabled={elements.length === 0 || isGenerating}>
        {isGenerating ? (
          <Spinner className="mr-2" />
        ) : (
          <IconPhoto className="mr-2" />
        )}
        {isGenerating ? "Generating..." : "Generate & Download Image"}
      </Button>
    </div>
  );
}
