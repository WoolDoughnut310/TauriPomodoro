<script lang="ts">
	import { appStore } from '$lib/stores';
	import Stat from '$lib/Stat.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import Stats from '$lib/Stats.svelte';

	let workTime = 25;
	let shortBreakTime = 5;
	let longBreakTime = 20;
	let longBreakInterval = 4;

	onMount(async () => {
		const settings = await appStore.get('settings');

		workTime = settings.work_time;
		shortBreakTime = settings.short_break_time;
		longBreakTime = settings.long_break_time;
		longBreakInterval = settings.long_break_interval;
	});

	const onSave = async () => {
		await invoke('update_settings', {
			settings: {
				work_time: workTime,
				short_break_time: shortBreakTime,
				long_break_time: longBreakTime,
				long_break_interval: longBreakInterval
			}
		});
	};
</script>

<Stats class="max-w-lg">
	<Stat title="Work" bind:value={workTime} input />
	<Stat title="Short break" bind:value={shortBreakTime} input />
	<Stat title="Long break" bind:value={longBreakTime} input />
</Stats>

<Stats class="w-48 mt-8">
	<Stat title="Long break interval" bind:value={longBreakInterval} input />
</Stats>

<div class="flex flex-row justify-end w-full mt-6">
	<button class="btn btn-primary" on:click={onSave}>Save</button>
</div>
