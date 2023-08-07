<script lang="ts">
	import { TimePhase, phase, sessionNumber } from './stores';

	const phaseNames = {
		[TimePhase.WORK]: 'Work',
		[TimePhase.SHORT_BREAK]: 'Short break',
		[TimePhase.LONG_BREAK]: 'Long break'
	};
	$: title = phaseNames[$phase];
</script>

<h1 class="text-3xl font-bold mb-2">{title}</h1>
<h3 class="text-xs font-semibold mb-8">Session {$sessionNumber}</h3>
