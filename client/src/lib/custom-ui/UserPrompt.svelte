<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
	import { onMount } from 'svelte';
	import { on } from 'svelte/events';

	const { prompt }: { prompt: string } = $props();

	let box: HTMLDivElement;

	onMount(() => {
		box.scrollIntoView({ behavior: 'smooth' });
	});
</script>

<div
	class="bg-secondary text-secondary-foreground px-4 py-2 rounded-lg shadow-md text-sm md:text-base"
	bind:this={box}
>
	<p>{prompt}</p>
</div>
