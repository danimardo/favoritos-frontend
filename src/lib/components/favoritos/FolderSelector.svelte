<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { Carpeta } from '$lib/types/api';

	let { value = $bindable("") } = $props();

	let folders = $state<Carpeta[]>([]);
	let isCreating = $state(false);
	let newFolderName = $state("");

	onMount(async () => {
		await loadFolders();
	});

	async function loadFolders() {
		try {
			folders = await invoke<Carpeta[]>('folders_get');
		} catch (e) {
			console.error('Error loading folders:', e);
		}
	}

	async function handleCreateFolder() {
		if (!newFolderName.trim()) return;
		try {
			const newFolder = await invoke<Carpeta>('folders_create', { name: newFolderName });
			folders = [...folders, newFolder];
			value = newFolder.id;
			isCreating = false;
			newFolderName = "";
		} catch (e) {
			console.error('Error creating folder:', e);
		}
	}

	let selectedLabel = $derived(
		Array.isArray(folders) ? (folders.find(f => f.id === value)?.name || "Sin carpeta") : "Sin carpeta"
	);
</script>

<div class="space-y-2">
	{#if isCreating}
		<div class="flex gap-2">
			<Input bind:value={newFolderName} placeholder="Nombre de la carpeta" autofocus />
			<Button variant="outline" size="sm" onclick={handleCreateFolder}>Añadir</Button>
			<Button variant="ghost" size="sm" onclick={() => isCreating = false}>❌</Button>
		</div>
	{:else}
		<div class="flex gap-2">
			<Select.Root type="single" bind:value>
				<Select.Trigger class="w-full bg-background border-silver">
					{selectedLabel}
				</Select.Trigger>
				<Select.Content class="bg-parchment border-silver">
					<Select.Item value="">Sin carpeta</Select.Item>
					{#each folders as folder}
						<Select.Item value={folder.id}>{folder.name}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
			<Button variant="outline" size="icon" onclick={() => isCreating = true} title="Nueva carpeta">
				➕
			</Button>
		</div>
	{/if}
</div>
