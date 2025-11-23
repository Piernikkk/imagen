import { ComponentProps, FC } from "react";
import { Dialogs } from "./dialogs";
import * as DialogPrimitive from "@radix-ui/react-dialog";

type DefaultDialogDefinition = {
  payload: unknown;
  returnValue: unknown;
};

export type DialogDefinition<
  T extends DefaultDialogDefinition = DefaultDialogDefinition,
> = {
  payload: T["payload"] | undefined;
  returnValue: T["returnValue"] | undefined;
};

export type DialogName = keyof Dialogs;
export type DialogPayload<T extends DialogName> = Dialogs[T]["payload"];
export type DialogState = "opened" | "closed";
export type DialogReturnValue<T extends DialogName> = Dialogs[T]["returnValue"];

type DialogStateItem<T extends DialogName> = {
  name: T;
  state: DialogState;
  payload: DialogPayload<T>;
  resolve: (value: DialogReturnValue<T>) => void;
};

export type DialogsStates = {
  [T in DialogName]?: DialogStateItem<T>;
};

export type TDialogContext = {
  show: <T extends DialogName>(
    name: T,
    payload?: DialogPayload<T>,
  ) => Promise<DialogReturnValue<T>>;
  hide: <T extends DialogName>(
    name: T,
    returnValue: DialogReturnValue<T>,
  ) => void;
};

export type DialogProps<T extends DialogName> = ComponentProps<
  typeof DialogPrimitive.Root
> & {
  payload: DialogPayload<T>;
  name: T;
};

export type DialogComponents = {
  [T in DialogName]: FC<DialogProps<T>>;
};
