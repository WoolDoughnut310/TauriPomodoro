import { writable } from "svelte/store";

interface Timer {
    paused: boolean;
    remaining: number;
    phase: "WORK" | "SHORT" | "LONG";
}

export const timer = writable<Timer>({
    paused: true,
    remaining: 0,
    phase: "WORK",
})

export const settings = writable({
    workTime: 25,
    shortBreakTime: 5,
    longBreakTime: 20,
    longBreakInterval: 4
})
