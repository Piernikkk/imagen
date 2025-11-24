import { Commands } from "@/lib/providers/types";
import { IconX } from "@tabler/icons-react";
import { Button } from "./ui/button";

interface TileProps {
  command: Commands;
  onRemove?: () => void;
}

export default function Tile({ command, onRemove }: TileProps) {
  const getCommandLabel = (cmd: Commands): string => {
    switch (cmd.type) {
      case "filled_rect":
        return "Rectangle";
      case "stroke_rect":
        return "Rectangle Outline";
      case "rounded_filled_rect":
        return "Rounded Rectangle";
      case "rounded_stroke_rect":
        return "Rounded Rectangle Outline";
      case "filled_circle":
        return "Circle";
      case "stroke_circle":
        return "Circle Outline";
      case "text":
        return "Text";
      case "pixel":
        return "Pixel";
      default:
        return "Unknown";
    }
  };

  const getCommandDescription = (cmd: Commands): string => {
    switch (cmd.type) {
      case "filled_rect":
      case "stroke_rect":
        return `Position: (${cmd.x}, ${cmd.y}), Size: ${cmd.width}×${cmd.height}${cmd.type === "stroke_rect" ? `, Stroke: ${cmd.thickness}px` : ""}`;
      case "rounded_filled_rect":
      case "rounded_stroke_rect":
        return `Position: (${cmd.x}, ${cmd.y}), Size: ${cmd.width}×${cmd.height}, Radius: ${cmd.radius}${cmd.type === "rounded_stroke_rect" ? `, Stroke: ${cmd.thickness}px` : ""}`;
      case "filled_circle":
      case "stroke_circle":
        return `Center: (${cmd.cx}, ${cmd.cy}), Radius: ${cmd.radius}${cmd.type === "stroke_circle" ? `, Stroke: ${cmd.thickness}px` : ""}`;
      case "text":
        return `"${cmd.text}" at (${cmd.x}, ${cmd.y}), Size: ${cmd.font_size}px`;
      case "pixel":
        return `Position: (${cmd.x}, ${cmd.y})`;
      default:
        return "";
    }
  };

  const getColor = (r: number, g: number, b: number, a: number): string => {
    return `rgba(${r}, ${g}, ${b}, ${a / 255})`;
  };

  const color = command.color;

  return (
    <div className="flex items-center gap-3 p-3 border rounded-lg bg-card hover:bg-accent/50 transition-colors group">
      <div
        className="w-4 h-4 rounded border border-border flex-shrink-0"
        style={{
          backgroundColor: getColor(color.r, color.g, color.b, color.a),
        }}
      />
      <div className="flex-1 min-w-0">
        <div className="font-semibold text-sm">{getCommandLabel(command)}</div>
        <div className="text-xs text-muted-foreground truncate">
          {getCommandDescription(command)}
        </div>
      </div>
      {onRemove && (
        <Button
          size="icon-sm"
          variant="ghost"
          onClick={onRemove}
          className="opacity-0 group-hover:opacity-100 transition-opacity flex-shrink-0">
          <IconX size={16} />
        </Button>
      )}
    </div>
  );
}
