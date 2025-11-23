import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Commands } from "@/lib/providers/types";
import { ColorPicker } from "./ColorPicker";

interface CommandFieldProps {
  field: string;
  command: Partial<Commands>;
  onFieldChange: (field: string, value: string | number) => void;
  onColorChange: (channel: "r" | "g" | "b" | "a", value: string) => void;
}

export function CommandField({
  field,
  command,
  onFieldChange,
  onColorChange,
}: CommandFieldProps) {
  if (field === "color") {
    const color = (typeof command.color === "object"
      ? command.color
      : null) || { r: 0, g: 0, b: 0, a: 255 };
    return <ColorPicker color={color} onChange={onColorChange} />;
  }

  if (field === "text") {
    return (
      <div className="space-y-2">
        <Label htmlFor={field} className="text-sm font-medium">
          Text
        </Label>
        <Input
          id={field}
          type="text"
          value={
            (command[field as keyof typeof command] as string | undefined) || ""
          }
          onChange={(e) => onFieldChange(field, e.target.value)}
          placeholder="Enter text to render"
        />
      </div>
    );
  }

  if (field === "font_size") {
    return (
      <div className="space-y-2">
        <Label htmlFor={field} className="text-sm font-medium">
          Font Size (pixels)
        </Label>
        <Input
          id={field}
          type="number"
          step="0.1"
          min="1"
          value={(() => {
            const val = command[field as keyof typeof command];
            return typeof val === "number" ? val : "";
          })()}
          onChange={(e) => onFieldChange(field, parseFloat(e.target.value))}
        />
      </div>
    );
  }

  return (
    <div className="space-y-2">
      <Label htmlFor={field} className="text-sm font-medium">
        {field.charAt(0).toUpperCase() + field.slice(1)}
      </Label>
      <Input
        id={field}
        type="number"
        min="0"
        value={(() => {
          const val = command[field as keyof typeof command];
          return typeof val === "number" ? val : "";
        })()}
        onChange={(e) => onFieldChange(field, parseInt(e.target.value))}
      />
    </div>
  );
}
