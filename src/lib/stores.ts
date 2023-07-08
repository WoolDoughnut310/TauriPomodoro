import { readable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import { Store } from '@tauri-apps/plugin-store';

const appStore = new Store('.store.dat');

enum TimePhase {
	WORK,
	SHORT_BREAK,
	LONG_BREAK
}

export const settings = readable(
	{
		workTime: 5,
		shortBreakTime: 5,
		longBreakTime: 20,
		longBreakInterval: 4
	},
	(set) => {
		let unlisten: () => void;
		appStore.onKeyChange('settings', (value) => set(value)).then((fn) => (unlisten = fn));
		return async () => {
			unlisten?.();
		};
	}
);

export const stats = readable({
	today: {
		minutes: 0,
		sessions: 0
	},
	week: {
		minutes: 0,
		sessions: 0
	},
	total: {
		minutes: 0,
		sessions: 0
	}
});

export const remaining = readable(0, (set, update) => {
	let unlisten: () => void;
	listen('remaining', (event) => {
		set(event.payload * 60);
	}).then((fn) => (unlisten = fn));

	const interval = setInterval(() => {
		update((previous) => previous - 1);
	});

	return () => {
		unlisten?.();
		clearInterval(interval);
	};
});

export const paused = readable(true, (set) => {
	let unlisten: () => void;
	listen('paused', (event) => {
		set(event.payload);
	}).then((fn) => (unlisten = fn));
	return () => {
		unlisten?.();
	};
});

export const phase = readable<TimePhase>(TimePhase.WORK, (set) => {
	let unlisten: () => void;
	listen('switch-phase', (event) => {
		set(event.payload);
	}).then((fn) => (unlisten = fn));
	return () => {
		unlisten?.();
	};
});
