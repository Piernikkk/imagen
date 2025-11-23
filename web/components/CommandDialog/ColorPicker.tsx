import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

interface ColorPickerProps {
  color: { r: number; g: number; b: number; a: number };
  onChange: (channel: "r" | "g" | "b" | "a", value: string) => void;
}

export function ColorPicker({ color, onChange }: ColorPickerProps) {
  return (
    <div className="space-y-2">
      <Label className="text-sm font-medium">Color (RGBA)</Label>
      <div className="grid grid-cols-4 gap-2">
        <div>
          <Label htmlFor="r" className="text-xs text-muted-foreground">
            R
          </Label>
          <Input
            id="r"
            type="number"
            min="0"
            max="255"
            value={color.r}
            onChange={(e) => onChange("r", e.target.value)}
            className="h-8"
          />
        </div>
        <div>
          <Label htmlFor="g" className="text-xs text-muted-foreground">
            G
          </Label>
          <Input
            id="g"
            type="number"
            min="0"
            max="255"
            value={color.g}
            onChange={(e) => onChange("g", e.target.value)}
            className="h-8"
          />
        </div>
        <div>
          <Label htmlFor="b" className="text-xs text-muted-foreground">
            B
          </Label>
          <Input
            id="b"
            type="number"
            min="0"
            max="255"
            value={color.b}
            onChange={(e) => onChange("b", e.target.value)}
            className="h-8"
          />
        </div>
        <div>
          <Label htmlFor="a" className="text-xs text-muted-foreground">
            A
          </Label>
          <Input
            id="a"
            type="number"
            min="0"
            max="255"
            value={color.a}
            onChange={(e) => onChange("a", e.target.value)}
            className="h-8"
          />
        </div>
      </div>
      <div
        className="w-full h-8 rounded border"
        style={{
          backgroundColor: `rgba(${color.r}, ${color.g}, ${color.b}, ${color.a / 255})`,
        }}
      />
    </div>
  );
}
