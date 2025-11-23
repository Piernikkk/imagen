import { useContext } from "react";
import { DialogContext } from ".";

export function useDialogs() {
  const context = useContext(DialogContext);

  if (!context) {
    throw new Error("useDialogs needs to be within DialogProvider");
  }

  return context;
}
