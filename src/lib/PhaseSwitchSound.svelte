<script lang="ts">
	import { TimePhase, phase, hasStarted } from '$lib/stores';
	import workSound from '$lib/assets/work.wav';
	import shortBreakSound from '$lib/assets/short-break.wav';
	import longBreakSound from '$lib/assets/long-break.wav';

	const sounds = {
		[TimePhase.WORK]: new Audio(workSound),
		[TimePhase.SHORT_BREAK]: new Audio(shortBreakSound),
		[TimePhase.LONG_BREAK]: new Audio(longBreakSound)
	};

	phase.subscribe((value) => {
		if (!$hasStarted) return;

		sounds[value].play();
	});
</script>
