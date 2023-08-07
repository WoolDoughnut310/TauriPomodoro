<script lang="ts">
	export let title: string;
	export let value: any;
	export let input = false;
</script>

<div class="stat place-items-center">
	{#if $$slots.figure}
		<div class="stat-figure text-secondary">
			<slot name="figure" />
		</div>
	{/if}
	<div class="stat-title">{title}</div>
	<div class="stat-value">
		{#if input}
			<input bind:value type="number" class="input input-ghost w-full max-w-xs text-center" />
		{:else}
			{value}
		{/if}
	</div>
</div>
