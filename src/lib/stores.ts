import { derived, readable, writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import { Store } from '@tauri-apps/plugin-store';
import { invoke } from '@tauri-apps/api';

export const appStore = new Store('.store.dat');

export enum TimePhase {
	WORK = 'Work',
	SHORT_BREAK = 'ShortBreak',
	LONG_BREAK = 'LongBreak'
}

export const paused = writable(true);
export const hasStarted = writable(false);

export const phase = readable<TimePhase>(TimePhase.WORK, (set) => {
	let unlisten: () => void;
	listen('switch-phase', (event) => {
		set(event.payload as TimePhase);
	}).then((fn) => (unlisten = fn));
	return () => {
		unlisten?.();
	};
});

export const sessionNumber = readable(1, (set) => {
	let unlisten: () => void;
	listen('session-number', (event) => {
		set((event.payload as number) + 1);
	}).then((fn) => (unlisten = fn));
	return () => {
		unlisten?.();
	};
});

export const remaining = derived(
	paused,
	($paused, set, update) => {
		let unlisten: () => void;
		listen('remaining', (event) => {
			set((event.payload as number) * 60);
		}).then((fn) => (unlisten = fn));

		const interval = setInterval(() => {
			if ($paused) return;
			update((previous) => {
				const newValue = Math.max(previous - 1, 0);
				if (newValue === 0) {
					invoke('switch_phase', { isPrevious: false, isUser: false });
				}

				return newValue;
			});
		}, 1000);

		return () => {
			unlisten?.();
			clearInterval(interval);
		};
	},
	25 * 60
);
