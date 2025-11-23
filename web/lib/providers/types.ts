import { paths } from "@/types/api";

export type Commands =
  paths["/api/build"]["post"]["requestBody"]["content"]["application/json"]["commands"][number];

export type CommandType = Commands["type"];
