<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import arrowLeft from '$lib/assets/arrow-left-circle.svg';
	import arrowRight from '$lib/assets/arrow-right-circle.svg';
	import { sessionNumber } from '$lib/stores';

	const switchPhase = (isPrevious: boolean) => {
		invoke('switch_phase', { isPrevious, isUser: true });
	};

	const btnClassName = 'btn btn-lg btn-circle btn-ghost';
</script>

<div class="flex flex-row w-full justify-center items-center">
	<div class="mr-4 w-16 h-16">
		{#if $sessionNumber > 1}
			<button class={btnClassName} on:click={() => switchPhase(true)}
				><img src={arrowLeft} alt="left" class="w-full h-full" /></button
			>
		{/if}
	</div>
	<slot />
	<button class={`${btnClassName} ml-4`} on:click={() => switchPhase(false)}
		><img src={arrowRight} alt="right" class="w-full h-full" /></button
	>
</div>
