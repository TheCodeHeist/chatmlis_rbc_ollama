<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';

	const { prompt }: { prompt: string } = $props();
</script>

<Card.Root>
	<Card.Content>
		<p>{prompt}</p>
	</Card.Content>
</Card.Root>
