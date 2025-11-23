import CommandDialog from "@/components/CommandDialog/index";
import { DialogComponents, DialogDefinition } from "./types";
import { Commands } from "../providers/types";

export type Dialogs = {
  Command: DialogDefinition<{
    payload: undefined;
    returnValue: Commands;
  }>;
};

export const DialogBindings: DialogComponents = {
  Command: CommandDialog,
};
