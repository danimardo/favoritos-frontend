<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';

	let { tags = $bindable([]), id = "" } = $props();
	let inputValue = $state("");

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			addTag();
		}
	}

	function addTag() {
		const trimmed = inputValue.trim();
		if (trimmed && !tags.includes(trimmed)) {
			tags = [...tags, trimmed];
			inputValue = "";
		}
	}

	function removeTag(tag: string) {
		tags = tags.filter(t => t !== tag);
	}
</script>

<div class="space-y-2">
	<div class="flex flex-wrap gap-2 mb-2">
		{#each tags as tag}
			<Badge variant="secondary" class="bg-powderBlush text-taupe gap-1 px-2 py-1">
				{tag}
				<button 
					class="hover:text-red-600 font-bold ml-1" 
					onclick={() => removeTag(tag)}
					type="button"
				>
					×
				</button>
			</Badge>
		{/each}
	</div>
	<div class="flex gap-2">
		<Input 
			id={id}
			bind:value={inputValue} 
			onkeydown={handleKeyDown} 
			placeholder="Añadir etiqueta y pulsar Enter" 
			class="bg-background border-silver"
		/>
		<Button variant="outline" onclick={addTag} type="button">Añadir</Button>
	</div>
</div>
