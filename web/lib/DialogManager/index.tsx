"use client";
import { createContext, FC, ReactNode, useCallback, useState } from "react";
import {
  DialogName,
  DialogPayload,
  DialogProps,
  DialogReturnValue,
  DialogsStates,
  TDialogContext,
} from "./types";
import { DialogBindings } from "./dialogs";

export const DialogContext = createContext<TDialogContext>({
  show: async () => undefined,
  hide: () => undefined,
});

export function DialogManager({ children }: { children: ReactNode }) {
  const [dialogs, setDialogs] = useState<DialogsStates>({});

  const show = useCallback(
    <T extends DialogName>(
      dialogName: T,
      dialogPayload: DialogPayload<T>,
    ): Promise<DialogReturnValue<T>> => {
      return new Promise((resolve) => {
        setDialogs((d) => ({
          ...d,
          [dialogName]: {
            name: dialogName,
            payload: dialogPayload,
            state: "opened",
            resolve,
          },
        }));
      });
    },
    [],
  );

  const hide = useCallback(
    <T extends DialogName>(
      dialogName: T,
      returnValue?: DialogReturnValue<T>,
    ) => {
      setDialogs((d) => {
        const dialog = d[dialogName];

        if (!dialog || dialog.state == "closed") {
          return d;
        }

        requestAnimationFrame(() => {
          (dialog.resolve as (value: DialogReturnValue<T>) => void)(
            returnValue,
          );
        });

        return { ...d, [dialogName]: { ...dialog, state: "closed" } };
      });
    },
    [],
  );

  return (
    <DialogContext.Provider value={{ show, hide }}>
      {Object.values(dialogs).map((d) => {
        const DialogComponent = DialogBindings[d.name] as FC<
          DialogProps<typeof d.name>
        >;

        return (
          <DialogComponent
            name={d.name}
            open={d.state == "opened"}
            onOpenChange={(open) => {
              if (!open) {
                hide(d.name);
              }
            }}
            key={d.name}
            payload={d.payload}
          />
        );
      })}
      {children}
    </DialogContext.Provider>
  );
}

export * from "./hook";
