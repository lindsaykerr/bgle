import type { Emulator, Games } from "./types";
import type { Writable } from "svelte/store";
import { writable, get } from "svelte/store";


export const emulatorListStore: Writable<Emulator[]> = writable([]);
export const gameListStore: Writable<Games> = writable({directory: "", emulator: "", games: []});
