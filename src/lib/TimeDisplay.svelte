<script lang="ts">
	import { remaining, phase, appStore } from '$lib/stores';

	const phaseTimeMapping = {
		Work: 'work_time',
		ShortBreak: 'short_break_time',
		LongBreak: 'long_break_time'
	};

	let total = 0;

	$: appStore.get('settings').then((value) => (total = value[phaseTimeMapping[$phase]] * 60));

	$: value = [Math.floor($remaining / 60), $remaining % 60];
	$: progress = ($remaining / total) * 100;
</script>

<div class="radial-progress" style="--size: 12rem; --value: {progress}">
	<span class="countdown font-mono text-2xl">
		<span style="--value: {value[0]}" />:
		<span style="--value: {value[1]}" />
	</span>
</div>
