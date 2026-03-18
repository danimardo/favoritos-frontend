<script lang="ts">
	import * as Command from "$lib/components/ui/command";
	import * as Popover from "$lib/components/ui/popover";
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import type { Carpeta } from "$lib/types/api";
	import { cn } from "$lib/utils";
	import { Check, ChevronsUpDown, Plus, Search, X } from "lucide-svelte";
	import { tick } from "svelte";

	let { value = $bindable("") } = $props();

	let open = $state(false);
	let folders = $state<any[]>([]);
	let isCreating = $state(false);
	let newFolderName = $state("");

	onMount(async () => {
		await loadFolders();
	});

	async function loadFolders() {
		try {
			// folders_get returns Vec<serde_json::Value>
			const rawFolders = await invoke<any[]>("folders_get");
			
			// Sort folders: first by item count (if available), then by name
			folders = rawFolders.sort((a, b) => {
				const countA = a.bookmarkCount || a.count || 0;
				const countB = b.bookmarkCount || b.count || 0;
				
				if (countB !== countA) {
					return countB - countA; // Descending
				}
				return a.name.localeCompare(b.name);
			});
		} catch (e) {
			console.error("Error loading folders:", e);
		}
	}

	async function handleCreateFolder() {
		if (!newFolderName.trim()) return;
		try {
			const newFolder = await invoke<Carpeta>("folders_create", { name: newFolderName });
			folders = [newFolder, ...folders];
			value = newFolder.id;
			isCreating = false;
			newFolderName = "";
			open = false;
		} catch (e) {
			console.error("Error creating folder:", e);
		}
	}

	let selectedLabel = $derived(
		value
			? (folders.find(f => String(f.id) === String(value))?.name || "Carpeta no encontrada")
			: "Sin carpeta"
	);

	function closeAndSelect(id: string | number) {
		value = String(id);
		open = false;
	}
</script>

<div class="space-y-2">
	{#if isCreating}
		<div class="flex gap-2">
			<Input 
				bind:value={newFolderName} 
				placeholder="Nombre de la nueva carpeta..." 
				class="bg-white border-silver text-taupe"
				autofocus 
				onkeydown={(e) => e.key === 'Enter' && handleCreateFolder()}
			/>
			<Button variant="outline" size="sm" onclick={handleCreateFolder} class="text-taupe border-silver">Añadir</Button>
			<Button variant="ghost" size="sm" onclick={() => isCreating = false} class="text-greyOlive">
				<X class="h-4 w-4" />
			</Button>
		</div>
	{:else}
		<div class="flex gap-2">
			<Popover.Root bind:open>
				<Popover.Trigger asChild>
					{#snippet children({ builder })}
						<Button
							builders={[builder]}
							variant="outline"
							role="combobox"
							aria-expanded={open}
							class="w-full justify-between bg-white border-silver text-taupe font-normal"
						>
							{selectedLabel}
							<ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
						</Button>
					{/snippet}
				</Popover.Trigger>
				<Popover.Content class="w-[300px] p-0 bg-white border-silver shadow-2xl z-[10000] opacity-100">
					<Command.Root>
						<div class="flex items-center border-b border-silver/30 px-3">
							<Search class="mr-2 h-4 w-4 shrink-0 opacity-50 text-taupe" />
							<Command.Input 
								placeholder="Buscar carpeta..." 
								class="h-9 text-taupe bg-transparent outline-none border-none focus:ring-0"
							/>
						</div>
						<Command.List class="max-h-[200px] overflow-y-auto p-1">
							<Command.Empty class="py-2 px-4 text-sm text-greyOlive">No se encontraron carpetas.</Command.Empty>
							<Command.Group>
								<Command.Item
									value=""
									onSelect={() => closeAndSelect("")}
									class="text-taupe flex items-center gap-2 px-2 py-1.5 cursor-pointer hover:bg-silver/20 aria-selected:bg-silver/20 rounded-sm"
								>
									<Check class={cn("h-4 w-4", value === "" ? "opacity-100" : "opacity-0")} />
									Sin carpeta
								</Command.Item>
								{#each folders as folder}
									<Command.Item
										value={folder.name}
										onSelect={() => closeAndSelect(folder.id)}
										class="text-taupe flex items-center justify-between px-2 py-1.5 cursor-pointer hover:bg-silver/20 aria-selected:bg-silver/20 rounded-sm"
									>
										<div class="flex items-center gap-2">
											<Check class={cn("h-4 w-4", String(value) === String(folder.id) ? "opacity-100" : "opacity-0")} />
											{folder.name}
										</div>
										{#if folder.bookmarkCount || folder.count}
											<span class="text-[10px] text-greyOlive bg-silver/10 px-1.5 rounded-full">
												{folder.bookmarkCount || folder.count}
											</span>
										{/if}
									</Command.Item>
								{/each}
							</Command.Group>
						</Command.List>
						<div class="border-t border-silver/30 p-1">
							<Button 
								variant="ghost" 
								class="w-full justify-start text-xs text-taupe hover:bg-silver/20 h-8"
								onclick={() => {
									isCreating = true;
									open = false;
								}}
							>
								<Plus class="mr-2 h-3 w-3" />
								Crear nueva carpeta
							</Button>
						</div>
					</Command.Root>
				</Popover.Content>
			</Popover.Root>
		</div>
	{/if}
</div>
