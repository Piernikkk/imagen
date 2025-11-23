import { CommandField } from "./CommandField";
import { Commands } from "@/lib/providers/types";

interface CommandFormProps {
  fields: string[];
  command: Partial<Commands>;
  onFieldChange: (field: string, value: string | number) => void;
  onColorChange: (channel: "r" | "g" | "b" | "a", value: string) => void;
}

export function CommandForm({
  fields,
  command,
  onFieldChange,
  onColorChange,
}: CommandFormProps) {
  return (
    <div className="space-y-4">
      <h4 className="font-medium text-sm">Command Properties</h4>
      {fields.map((field) => (
        <CommandField
          key={field}
          field={field}
          command={command}
          onFieldChange={onFieldChange}
          onColorChange={onColorChange}
        />
      ))}
    </div>
  );
}
