import { writable } from "svelte/store";
import type { ResponseTask } from "./routes/dashboard/tasks";

export const tasks = writable<ResponseTask[]>([]);
export const username = writable<string>("");
