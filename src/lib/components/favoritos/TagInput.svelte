<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';

	let { tags = $bindable([]), id = "" } = $props();
	let inputValue = $state("");

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ',') {
			e.preventDefault();
			addTag();
		}
	}

	function addTag() {
		// Split by comma in case multiple tags were pasted or typed
		const newTags = inputValue
			.split(',')
			.map(t => t.trim())
			.filter(t => t !== "" && !tags.includes(t));

		if (newTags.length > 0) {
			tags = [...tags, ...newTags];
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
			placeholder="Ej: trabajo, dev..." 
			class="bg-background border-silver"
		/>
		<Button variant="outline" onclick={addTag} type="button" class="text-taupe border-silver">Añadir</Button>
	</div>
	<p class="text-[10px] text-greyOlive">Separa las etiquetas con comas o pulsa Enter para añadirlas.</p>
</div>
