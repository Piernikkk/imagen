import { useState } from "react";
import { Commands } from "@/lib/providers/types";

interface CommandTypeInfo {
  value: string;
  label: string;
  description: string;
  fields: string[];
}

export function useCommandForm() {
  const [selectedCommand, setSelectedCommand] = useState<string>("");
  const [command, setCommand] = useState<Partial<Commands>>({});

  const handleCommandTypeChange = (value: string) => {
    setSelectedCommand(value);
    if (value) {
      const initialCommand: Partial<Commands> = {
        type: value,
        color: { r: 0, g: 0, b: 0, a: 255 },
      } as Partial<Commands>;
      setCommand(initialCommand);
    } else {
      setCommand({});
    }
  };

  const handleFieldChange = (field: string, value: string | number) => {
    setCommand((prev) => ({
      ...prev,
      [field]: value,
    }));
  };

  const handleColorChange = (channel: "r" | "g" | "b" | "a", value: string) => {
    const numValue = Math.min(255, Math.max(0, parseInt(value) || 0));
    setCommand((prev) => ({
      ...prev,
      color: {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
        ...(typeof prev.color === "object" ? prev.color : {}),
        [channel]: numValue,
      },
    }));
  };

  const isFormValid = (selectedCommandInfo?: CommandTypeInfo) => {
    if (!selectedCommandInfo) return false;

    for (const field of selectedCommandInfo.fields) {
      if (field === "color") {
        const color = command.color;
        if (
          !color ||
          typeof color !== "object" ||
          typeof color.r !== "number" ||
          typeof color.g !== "number" ||
          typeof color.b !== "number" ||
          typeof color.a !== "number"
        ) {
          return false;
        }
      } else {
        const value = command[field as keyof typeof command];
        if (value === undefined || value === null) {
          return false;
        }
        if (typeof value === "string" && value.trim() === "") {
          return false;
        }
      }
    }
    return true;
  };

  const resetForm = () => {
    setSelectedCommand("");
    setCommand({});
  };

  return {
    selectedCommand,
    command,
    handleCommandTypeChange,
    handleFieldChange,
    handleColorChange,
    isFormValid,
    resetForm,
  };
}
