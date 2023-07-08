<script lang="ts">
	import { remaining, phase, settings } from '$lib/stores';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';

	let total: number;

	onMount(() => {
		const phaseTimeMapping = {
			WORK: 'work_time',
			SHORT_BREAK: 'short_break_time',
			LONG_BREAK: 'long_break_time'
		};
		total = get(settings)[phaseTimeMapping[get(phase)]];
	});

	$: value = [Math.floor($remaining / 60), $remaining % 60];
	$: progress = ((total - $remaining) / total) * 100; // TO update
</script>

{@debug total, value}
<div class="radial-progress" style="--size: 12rem; --value: {progress}">
	<span class="countdown font-mono text-2xl">
		<span style="--value: {value[0]}" />:
		<span style="--value: {value[1]}" />
	</span>
</div>
