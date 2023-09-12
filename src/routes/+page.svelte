<script>
	import TimeDisplay from '$lib/TimeDisplay.svelte';
	import ActionButton from '$lib/ActionButton.svelte';
	import PhaseSwitcher from '$lib/PhaseSwitcher.svelte';
	import PhaseTitle from '$lib/PhaseTitle.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { appWindow } from '@tauri-apps/api/window';
	import { phase, remaining } from '$lib/stores';

	$: parsedRemaining = [Math.floor($remaining / 60), $remaining % 60].map((n) =>
		String(n).padStart(2, '0')
	);

	$: appWindow.setTitle(`${$phase} - ${parsedRemaining.join(':')} - Tauri Pomodor`);
</script>

<PhaseTitle />
<PhaseSwitcher><TimeDisplay /></PhaseSwitcher>
<ActionButton />
