<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { favoritos } from '$lib/stores/favoritos';
	import { Search, Plus } from 'lucide-svelte';

	let { tags = $bindable([]), id = "" } = $props();
	let inputValue = $state("");
	let showSuggestions = $state(false);
	let selectedIndex = $state(-1);
	let inputRef = $state<HTMLInputElement | null>(null);

	// Normalize for accent-insensitive matching
	function normalize(s: string): string {
		return s.normalize('NFD').replace(/[\u0300-\u036f]/g, '').toLowerCase();
	}

	let suggestions = $derived.by(() => {
		const query = normalize(inputValue.trim());
		if (!query) return [];
		return $favoritos.allTags
			.filter(t => normalize(t.name).includes(query) && !tags.includes(t.name))
			.sort((a, b) => (b.bookmarkCount ?? 0) - (a.bookmarkCount ?? 0))
			.slice(0, 10);
	});

	let exactMatch = $derived(
		$favoritos.allTags.some(t => normalize(t.name) === normalize(inputValue.trim()))
	);

	let canCreateNew = $derived(
		inputValue.trim() !== '' && !exactMatch && !tags.includes(inputValue.trim())
	);

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			const max = suggestions.length + (canCreateNew ? 1 : 0);
			if (max > 0) {
				selectedIndex = (selectedIndex + 1) % max;
				showSuggestions = true;
			}
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			const max = suggestions.length + (canCreateNew ? 1 : 0);
			if (max > 0) {
				selectedIndex = (selectedIndex - 1 + max) % max;
				showSuggestions = true;
			}
		} else if (e.key === 'Enter') {
			e.preventDefault();
			if (showSuggestions && selectedIndex >= 0 && selectedIndex < suggestions.length) {
				selectSuggestion(suggestions[selectedIndex].name);
			} else if (showSuggestions && canCreateNew && selectedIndex === suggestions.length) {
				addTag();
			} else {
				addTag();
			}
		} else if (e.key === ',') {
			e.preventDefault();
			addTag();
		} else if (e.key === 'Escape') {
			showSuggestions = false;
			selectedIndex = -1;
		}
	}

	function selectSuggestion(name: string) {
		if (!tags.includes(name)) {
			tags = [...tags, name];
		}
		inputValue = "";
		showSuggestions = false;
		selectedIndex = -1;
		inputRef?.focus();
	}

	function addTag() {
		const newTags = inputValue
			.split(',')
			.map(t => t.trim())
			.filter(t => t !== "" && !tags.includes(t));

		if (newTags.length > 0) {
			tags = [...tags, ...newTags];
			inputValue = "";
		}
		showSuggestions = false;
		selectedIndex = -1;
	}

	function removeTag(tag: string) {
		tags = tags.filter(t => t !== tag);
	}

	function handleInput() {
		showSuggestions = inputValue.trim().length > 0;
		selectedIndex = -1;
	}

	function handleBlur() {
		// Delay to allow click on suggestion
		setTimeout(() => {
			showSuggestions = false;
			selectedIndex = -1;
		}, 200);
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
	<div class="relative">
		<div class="flex gap-2">
			<Input
				id={id}
				bind:value={inputValue}
				bind:ref={inputRef}
				onkeydown={handleKeyDown}
				oninput={handleInput}
				onfocus={handleInput}
				onblur={handleBlur}
				placeholder="Buscar o crear etiqueta..."
				class="bg-background border-silver"
				autocomplete="off"
			/>
			<Button variant="outline" onclick={addTag} type="button" class="text-taupe border-silver">Añadir</Button>
		</div>
		{#if showSuggestions && (suggestions.length > 0 || canCreateNew)}
			<div class="absolute z-50 w-full mt-1 bg-white border border-silver rounded-md shadow-lg max-h-[200px] overflow-y-auto">
				{#each suggestions as suggestion, i}
					<button
						type="button"
						class="w-full flex items-center justify-between px-3 py-2 text-sm text-left hover:bg-silver/20 transition-colors {selectedIndex === i ? 'bg-silver/20' : ''}"
						onmousedown={() => selectSuggestion(suggestion.name)}
					>
						<div class="flex items-center gap-2">
							<Search class="w-3 h-3 text-greyOlive shrink-0" />
							<span class="text-taupe">{suggestion.name}</span>
						</div>
						{#if suggestion.bookmarkCount}
							<span class="text-[10px] text-greyOlive bg-silver/10 px-1.5 rounded-full">
								{suggestion.bookmarkCount}
							</span>
						{/if}
					</button>
				{/each}
				{#if canCreateNew}
					<button
						type="button"
						class="w-full flex items-center gap-2 px-3 py-2 text-sm text-left hover:bg-silver/20 transition-colors border-t border-silver/30 {selectedIndex === suggestions.length ? 'bg-silver/20' : ''}"
						onmousedown={addTag}
					>
						<Plus class="w-3 h-3 text-taupe shrink-0" />
						<span class="text-taupe">Crear "<strong>{inputValue.trim()}</strong>"</span>
					</button>
				{/if}
			</div>
		{/if}
	</div>
	<p class="text-[10px] text-greyOlive">Escribe para buscar etiquetas existentes o crear nuevas.</p>
</div>
