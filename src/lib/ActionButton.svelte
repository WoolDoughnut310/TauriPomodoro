<script lang="ts">
	import { paused, hasStarted } from '$lib/stores';
	import { invoke } from '@tauri-apps/api/tauri';

	const onClick = () => {
		$paused = !$paused;
		if (!$hasStarted) $hasStarted = true;
	};

	const onReset = async () => {
		await invoke('reset_phase');
	};

	$: canReset = $paused && $hasStarted;

	$: btnClassName = 'mt-10 btn btn-primary ' + (canReset ? 'w-32' : 'w-64');
</script>

<div class="flex flex-row w-full justify-center items-center space-x-6">
	{#if canReset}
		<button class={btnClassName} on:click={onReset}>Reset</button>
	{/if}
	<button class={btnClassName} on:click={onClick}>
		{#if $paused}{$hasStarted ? 'Resume' : 'Start'}{:else}Pause{/if}
	</button>
</div>
