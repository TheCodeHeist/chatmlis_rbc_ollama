<script lang="ts">
	import { Input } from '$lib/components/ui/input/index.js';
	import { Button, buttonVariants } from '$lib/components/ui/button/index.js';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import { GitBranchIcon, LoaderCircle, SendHorizontal } from 'lucide-svelte';
	import ThemeSwitch from '$lib/custom-ui/ThemeSwitch.svelte';
	import UserPrompt from '$lib/custom-ui/UserPrompt.svelte';
	import BotResponse from '$lib/custom-ui/BotResponse.svelte';
	import { fade } from 'svelte/transition';

	let chatInput = $state('');
	let isChatLoading = $state(false);
	let chatHistory: { role: 'user' | 'assistant'; content: string }[] = $state([
		// {
		// 	role: 'user',
		// 	content: 'Tell me about Maple Leaf.'
		// },
		// {
		// 	role: 'assistant',
		// 	content: `Okay, let's dive into Maple Leaf International School! I’ve been analyzing...`
		// }
	]);

	async function handleSend(e: MouseEvent) {
		e.preventDefault();

		if (chatInput.trim() === '') return;

		// Add user input to chat history
		chatHistory.push({ role: 'user', content: chatInput });
		chatInput = '';

		isChatLoading = true;

		// Simulate a bot response (replace with actual API call)
		const botResponse = await fetch('http://127.0.0.1:3000/api/chat', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ messages: chatHistory })
		}).then((res) => res.json());

		// Add bot response to chat history
		chatHistory.push(botResponse.message);
		isChatLoading = false;

		// Clear input field
	}
</script>

<main class="h-screen w-full flex flex-col items-center">
	<header class="flex items-center justify-between py-4 px-8 fixed top-0 w-full">
		<div class="flex items-center gap-4">
			<p class="text-2xl font-semibold text-primary">ChatMLIS</p>
			<Badge variant="secondary">Google's Gemma3</Badge>
		</div>

		<div class="flex items-center gap-4">
			<a
				class={buttonVariants({ variant: 'ghost' })}
				href="https://github.com/TheCodeHeist/chatmlis_rbc_ollama"
				target="_blank"
				rel="noopener noreferrer"
				aria-label="GitHub Repository"
			>
				<GitBranchIcon />
				<span>Source Code</span>
			</a>

			<ThemeSwitch />
		</div>
	</header>

	<article class="h-full w-1/2 flex items-center justify-center flex-col gap-8 py-32">
		{#if chatHistory.length === 0}
			<p class="text-xl">How can I assist you today?</p>
		{/if}

		<!-- Chats -->
		{#if chatHistory.length !== 0}
			<section class="flex-1 overflow-y-auto p-4 w-full flex flex-col items-center gap-8">
				{#each chatHistory as chat}
					{#if chat.role === 'user'}
						<div class="flex justify-end w-full" in:fade|global>
							<UserPrompt prompt={chat.content} />
						</div>
					{:else}
						<div class="flex justify-start w-full" in:fade|global>
							<BotResponse response={chat.content} />
						</div>
					{/if}
				{/each}

				{#if isChatLoading}
					<div class="flex justify-start items-center w-full gap-2" transition:fade|global>
						<LoaderCircle class="animate-spin text-primary" />
						<p class="text-primary">Thinking...</p>
					</div>
				{/if}
			</section>
		{/if}

		<div
			class={'flex items-center justify-center gap-2 ' +
				(chatHistory.length === 0 ? 'w-1/2' : 'w-full')}
		>
			<Input type="text" placeholder="Ask anything" class="w-full" bind:value={chatInput} />
			<Button size="icon" class="size-8 cursor-pointer" onclick={handleSend}>
				<SendHorizontal />
			</Button>
		</div>
	</article>

	<footer class="text-center py-4 fixed w-full bottom-0">
		<p class="">Made with ❤️ by MLIS Robotics Club</p>
	</footer>
</main>
