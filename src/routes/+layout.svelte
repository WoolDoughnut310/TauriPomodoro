<script>
	import '../app.css';
	import { onMount } from 'svelte';
	import { themeChange } from 'theme-change';
	import PhaseSwitchSound from '$lib/PhaseSwitchSound.svelte';
	import { remaining } from '$lib/stores';
	import { invoke } from '@tauri-apps/api';

	onMount(async () => {
		themeChange(false);
		await invoke('restore_state');
	});

	const themes = [
		'light',
		'dark',
		'cupcake',
		'bumblebee',
		'emerald',
		'corporate',
		'synthwave',
		'retro',
		'cyberpunk',
		'valentine',
		'halloween',
		'garden',
		'forest',
		'aqua',
		'lofi',
		'pastel',
		'fantasy',
		'wireframe',
		'black',
		'luxury',
		'dracula',
		'cmyk',
		'autumn',
		'business',
		'acid',
		'lemonade',
		'night',
		'coffee',
		'winter'
	];

	$remaining;
</script>

<div class="bg-base-100 flex min-h-screen flex-col">
	<div class="navbar bg-neutral text-neutral-content px-6 py-10 justify-between">
		<h1 class="text-3xl font-bold">Tauri Pomodor</h1>
		<div class="flex flex-row space-x-4">
			<a role="button" href="/" class="btn btn-active">Timer</a>
			<a role="button" href="/report" class="btn btn-active">Report</a>
			<a role="button" href="/settings" class="btn btn-active">Settings</a>
		</div>
		<select data-choose-theme class="select select-ghost">
			<option value="" disabled selected>Theme</option>
			{#each themes as theme}
				<option value={theme}>{theme[0].toUpperCase() + theme.slice(1)}</option>
			{/each}
		</select>
	</div>
	<div class="flex flex-col p-8 flex-1 justify-center items-center"><slot /></div>
</div>
<PhaseSwitchSound />
