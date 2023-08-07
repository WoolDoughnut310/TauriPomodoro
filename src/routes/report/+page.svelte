<script lang="ts">
	import { appStore } from '$lib/stores';
	import Stat from '$lib/Stat.svelte';
	import Stats from '$lib/Stats.svelte';
	import clock from '$lib/assets/clock.svg?raw';
	import hash from '$lib/assets/hash.svg?raw';
	import { onMount } from 'svelte';

	let stats = {
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
	};

	$: appStore.get('stats').then((value) => (stats = value));

	const timeScaleOptions = ['today', 'week', 'total'];
	let timeScale = 'today';

	const getIdForOption = (name: string) => `${name}-input`;

	onMount(() => {
		(document.getElementById(getIdForOption('today')) as HTMLInputElement)?.click();
	});
</script>

<div class="join" on:change={(e) => (timeScale = e.target?.value)}>
	{#each timeScaleOptions as timeScaleOption}
		<input
			class="join-item btn"
			type="radio"
			name="options"
			id={getIdForOption(timeScaleOption)}
			value={timeScaleOption}
			aria-label={timeScaleOption[0].toLowerCase() + timeScaleOption.slice(1)}
		/>
	{/each}
</div>

<Stats class="mt-6 text-primary">
	<Stat title="Minutes" value={stats[timeScale].minutes}
		><div class="text-primary contents" slot="figure">{@html clock}</div></Stat
	>
	<Stat title="Sessions" value={stats[timeScale].sessions}
		><div class="text-primary contents" slot="figure">{@html hash}</div></Stat
	>
</Stats>
