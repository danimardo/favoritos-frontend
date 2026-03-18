<script lang="ts">
	import { onMount } from 'svelte';

	let {
		tags,
		onTagClick
	}: {
		tags: Array<string | { id: string; name: string; color?: string }>;
		onTagClick?: (tag: string | { id: string; name: string; color?: string }) => void;
	} = $props();

	let containerRef = $state<HTMLDivElement | null>(null);
	let visibleCount = $state(tags.length);

	function getTagName(tag: string | { name: string }): string {
		return typeof tag === 'string' ? tag : tag.name;
	}

	function recalculate() {
		if (!containerRef) return;
		const children = containerRef.querySelectorAll<HTMLElement>('[data-tag]');
		const containerRight = containerRef.getBoundingClientRect().right;
		// Reserve space for the "+N" badge (~30px)
		const badgeSpace = 30;
		let count = 0;

		for (const child of children) {
			child.style.display = '';
			const childRight = child.getBoundingClientRect().right;
			if (childRight + badgeSpace > containerRight && count < children.length) {
				break;
			}
			count++;
		}

		// If all tags fit without needing a badge, no need to reserve badge space
		if (count === children.length) {
			visibleCount = count;
		} else {
			// Re-check: the last visible might overflow when we don't need badge space
			visibleCount = count;
		}

		// Hide overflowing tags
		children.forEach((child, i) => {
			child.style.display = i < visibleCount ? '' : 'none';
		});
	}

	onMount(() => {
		if (!containerRef) return;
		const observer = new ResizeObserver(() => {
			// Reset visibility before recalculating
			const children = containerRef!.querySelectorAll<HTMLElement>('[data-tag]');
			children.forEach(c => c.style.display = '');
			visibleCount = tags.length;
			requestAnimationFrame(recalculate);
		});
		observer.observe(containerRef);
		requestAnimationFrame(recalculate);
		return () => observer.disconnect();
	});

	$effect(() => {
		// Re-run when tags change
		tags;
		if (containerRef) {
			requestAnimationFrame(recalculate);
		}
	});

	let hiddenCount = $derived(tags.length - visibleCount);
</script>

<div bind:this={containerRef} class="flex gap-1 overflow-hidden items-center" style="flex-wrap: nowrap;">
	{#each tags as tag}
		<button
			data-tag
			onclick={(e) => { e.stopPropagation(); if (onTagClick) onTagClick(tag); }}
			class="px-1.5 py-0.5 rounded bg-silver/20 text-[9px] text-taupe border border-silver/30 hover:bg-taupe hover:text-parchment transition-colors whitespace-nowrap shrink-0"
		>
			{getTagName(tag)}
		</button>
	{/each}
	{#if hiddenCount > 0}
		<span class="text-[9px] text-greyOlive shrink-0">+{hiddenCount}</span>
	{/if}
</div>
