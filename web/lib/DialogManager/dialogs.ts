import CommandDialog from "@/components/CommandDialog";
import { DialogComponents, DialogDefinition } from "./types";

export type Dialogs = {
  Command: DialogDefinition<{
    payload: undefined;
    returnValue: undefined;
  }>;
};

export const DialogBindings: DialogComponents = {
  Command: CommandDialog,
};
