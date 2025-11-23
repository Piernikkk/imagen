import { DialogProps } from "@/lib/DialogManager/types";
import { Dialog, DialogContent, DialogTitle } from "../ui/dialog";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { Button } from "../ui/button";
import { Combobox } from "../ui/combobox";
import { $api } from "@/lib/providers/api";
import { Commands } from "@/lib/providers/types";
import { useCommandForm } from "./useCommandForm";
import { CommandForm } from "./CommandForm";
import { useDialogs } from "@/lib/DialogManager";

interface CommandTypeInfo {
  value: string;
  label: string;
  description: string;
  fields: string[];
}

interface CommandTypesResponse {
  commands: CommandTypeInfo[];
}

export default function CommandDialog({
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Root> & DialogProps<"Command">) {
  const dialogs = useDialogs();

  const {
    selectedCommand,
    command,
    handleCommandTypeChange,
    handleFieldChange,
    handleColorChange,
    isFormValid,
  } = useCommandForm();

  const { data: commandTypesData, isLoading } = $api.useQuery(
    "get",
    "/api/commands",
  );

  const responseData = commandTypesData as unknown as
    | CommandTypesResponse
    | undefined;
  const commands = responseData?.commands || [];

  const commandOptions = commands.map((cmd) => ({
    value: cmd.value,
    label: cmd.label,
  }));

  const selectedCommandInfo = commands.find((c) => c.value === selectedCommand);

  const handleAddCommand = () => {
    if (isFormValid(selectedCommandInfo) && command.type) {
      dialogs.hide("Command", command as Commands);
    }
  };

  return (
    <Dialog {...props}>
      <DialogContent className="sm:max-w-3xl h-[600px] max-h-[600px] dark:bg-background/50 bg-background/70 backdrop-blur-sm overflow-hidden flex flex-col">
        <DialogTitle>Add Drawing Command</DialogTitle>
        <div className="flex flex-col w-full flex-1 overflow-hidden">
          <div className="flex justify-between items-center pb-4">
            <div className="font-bold">Select a drawing command:</div>
            <div className="flex gap-1">
              <Combobox
                selectLabel={isLoading ? "Loading..." : "Select command type"}
                searchLabel="Search commands..."
                data={commandOptions}
                value={selectedCommand}
                setValue={(value) => {
                  if (typeof value === "string") {
                    handleCommandTypeChange(value);
                  }
                }}
              />
            </div>
          </div>

          {selectedCommand && selectedCommandInfo && (
            <div className="flex-1 overflow-y-auto">
              <div className="p-4 border rounded-md mb-4">
                <h3 className="font-semibold mb-1">
                  {selectedCommandInfo.label}
                </h3>
                <p className="text-sm text-muted-foreground">
                  {selectedCommandInfo.description}
                </p>
              </div>

              <CommandForm
                fields={selectedCommandInfo.fields}
                command={command}
                onFieldChange={handleFieldChange}
                onColorChange={handleColorChange}
              />
            </div>
          )}

          {!selectedCommand && (
            <div className="flex-1 flex items-center justify-center text-muted-foreground">
              Select a command type to get started
            </div>
          )}
        </div>

        <div className="pt-4 flex justify-end gap-2 border-t">
          <Button variant="outline" onClick={() => props.onOpenChange?.(false)}>
            Cancel
          </Button>
          <Button
            disabled={!isFormValid(selectedCommandInfo)}
            onClick={handleAddCommand}>
            Add Command
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
