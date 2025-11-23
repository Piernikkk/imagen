import { DialogProps } from "@/lib/DialogManager/types";
import { Dialog, DialogContent, DialogTitle } from "./ui/dialog";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { Button } from "./ui/button";
import { Combobox } from "./ui/combobox";
import { useState } from "react";
import { $api } from "@/lib/providers/api";

export default function CommandDialog({
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Root> & DialogProps<"Command">) {
  const [selectedCommand, setSelectedCommand] = useState<string>("");

  // const [c]

  const { data: commandTypesData, isLoading } = $api.useQuery(
    "get",
    "/api/commands",
  );

  const commandOptions =
    commandTypesData?.commands.map((cmd) => ({
      value: cmd.value,
      label: cmd.label,
    })) || [];

  return (
    <Dialog {...props}>
      <DialogContent className="sm:max-w-3xl h-150 max-h-150 dark:bg-background/50 bg-background/70 backdrop-blur-sm overflow-hidden flex flex-col">
        <DialogTitle>Select Command</DialogTitle>
        <div className="flex flex-col w-full flex-1">
          <div className="flex justify-between items-center pb-2">
            <div className="font-bold">Select a drawing command:</div>
            <div className="flex gap-1">
              <Combobox
                selectLabel={isLoading ? "Loading..." : "Select command type"}
                searchLabel="Search commands..."
                data={commandOptions}
                value={selectedCommand}
                setValue={setSelectedCommand}
              />
            </div>
          </div>
          <div className="mt-4 p-4 border rounded-md flex-1">
            {selectedCommand && commandTypesData && (
              <>
                <h3 className="font-semibold mb-2">
                  {
                    commandTypesData.commands.find(
                      (c) => c.value === selectedCommand,
                    )?.label
                  }
                </h3>
                <p className="text-sm text-muted-foreground mb-2">
                  {
                    commandTypesData.commands.find(
                      (c) => c.value === selectedCommand,
                    )?.description
                  }
                </p>
                <div className="text-sm">
                  <span className="font-medium">Required fields:</span>{" "}
                  {commandTypesData.commands
                    .find((c) => c.value === selectedCommand)
                    ?.fields.join(", ")}
                </div>
              </>
            )}
          </div>
          <div className=" pt-4 flex justify-end">
            <Button
              disabled={selectedCommand == ""}
              onClick={() =>
                console.log("selected command:", { selectedCommand })
              }>
              Add Command
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
