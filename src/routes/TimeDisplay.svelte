<script lang="ts">
    import { settings } from "~lib/stores";
    import { onMount } from "svelte";

    let remaining = settings.remaining;

    let value = [Math.floor(remaining / 60), remaining % 60];
    let progress = remaining / settings.workTime; // TO update

    onMount(() => {
        setInterval(() => {
            remaining -= 1;
        }, 1000);
    })
</script>

<div class="radial-progress" style="--value: {progress}">
    <span class="countdown font-mono text-2xl">
        <span style="--value: {value[0]}"></span>;
        <span style="--value: {value[1]}"></span>
    </span>
</div>
