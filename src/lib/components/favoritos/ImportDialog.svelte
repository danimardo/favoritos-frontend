<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { invoke } from '@tauri-apps/api/core';
	import { tauriNotifications } from '$lib/services/tauri';
	import { FileJson, Upload, CheckCircle2, AlertCircle, Loader2 } from 'lucide-svelte';
	import { favoritos } from '$lib/stores/favoritos';

	let { open = $bindable(false) } = $props();

	let fileInput = $state<HTMLInputElement | null>(null);
	let file = $state<File | null>(null);
	let loading = $state(false);
	let result = $state<{ imported: number; skipped: number; errors: string[] } | null>(null);
	let error = $state<string | null>(null);

	function handleFileChange(e: Event) {
		const target = e.target as HTMLInputElement;
		if (target.files && target.files.length > 0) {
			file = target.files[0];
			error = null;
			result = null;
		}
	}

	async function startImport() {
		if (!file) return;

		loading = true;
		error = null;

		try {
			const text = await file.text();
			
			// Basic client-side validation
			const data = JSON.parse(text);
			if (!Array.isArray(data)) {
				throw new Error('El archivo debe ser un array de favoritos.');
			}

			const importResult = await invoke('import_favorites', { data: text }) as any;
			result = importResult;
			
			if (result && result.imported > 0) {
				await favoritos.fetchAll(); // Refresh list
				await tauriNotifications.showSuccess(`Importación completada: ${result.imported} favoritos añadidos.`);
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Error al importar el archivo.';
			console.error(e);
		} finally {
			loading = false;
		}
	}

	function reset() {
		file = null;
		result = null;
		error = null;
		if (fileInput) fileInput.value = '';
	}
</script>

<Dialog.Root bind:open onOpenChange={(o) => !o && reset()}>
	<Dialog.Content class="sm:max-w-[425px] bg-parchment border-taupe">
		<Dialog.Header>
			<Dialog.Title class="text-taupe flex items-center gap-2">
				<FileJson class="h-5 w-5" />
				Importar Favoritos
			</Dialog.Title>
			<Dialog.Description class="text-grey-olive">
				Sube un archivo JSON con tu colección de favoritos para importarlos masivamente.
			</Dialog.Description>
		</Dialog.Header>

		<div class="grid gap-4 py-4">
			{#if !result}
				<div class="grid w-full items-center gap-1.5">
					<Label for="json-file" class="text-taupe">Archivo JSON</Label>
					<div 
						class="flex flex-col items-center justify-center border-2 border-dashed border-silver rounded-lg p-6 bg-taupe/5 hover:bg-taupe/10 transition-colors cursor-pointer"
						onclick={() => fileInput?.click()}
						onkeydown={(e) => e.key === 'Enter' && fileInput?.click()}
						role="button"
						tabindex="0"
					>
						<Upload class="h-8 w-8 text-grey-olive mb-2" />
						<p class="text-sm text-grey-olive">
							{file ? file.name : 'Haz clic para seleccionar o arrastra el archivo'}
						</p>
						<input
							id="json-file"
							type="file"
							accept=".json"
							class="hidden"
							bind:this={fileInput}
							onchange={handleFileChange}
						/>
					</div>
					{#if error}
						<p class="text-xs text-red-500 mt-1 flex items-center gap-1">
							<AlertCircle class="h-3 w-3" />
							{error}
						</p>
					{/if}
				</div>
			{:else}
				<div class="bg-taupe/5 rounded-lg p-4 space-y-3">
					<div class="flex items-center gap-2 text-green-600 font-medium">
						<CheckCircle2 class="h-5 w-5" />
						Importación finalizada
					</div>
					<div class="grid grid-cols-2 gap-4 text-sm">
						<div class="bg-white/50 p-2 rounded border border-silver">
							<p class="text-grey-olive">Importados</p>
							<p class="text-2xl font-bold text-taupe">{result.imported}</p>
						</div>
						<div class="bg-white/50 p-2 rounded border border-silver">
							<p class="text-grey-olive">Saltados/Error</p>
							<p class="text-2xl font-bold text-taupe">{result.skipped}</p>
						</div>
					</div>
					{#if result.errors.length > 0}
						<div class="max-h-24 overflow-y-auto mt-2">
							<p class="text-xs font-semibold text-taupe mb-1">Errores:</p>
							<ul class="text-[10px] text-red-500 list-disc pl-4">
								{#each result.errors.slice(0, 5) as err}
									<li>{err}</li>
								{/each}
								{#if result.errors.length > 5}
									<li>... y {result.errors.length - 5} más</li>
								{/if}
							</ul>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<Dialog.Footer>
			{#if !result}
				<Button 
					variant="outline" 
					onclick={() => (open = false)}
					disabled={loading}
					class="border-taupe text-taupe hover:bg-taupe/10"
				>
					Cancelar
				</Button>
				<Button 
					onclick={startImport} 
					disabled={!file || loading}
					class="bg-taupe text-parchment hover:bg-taupe/90"
				>
					{#if loading}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
						Importando...
					{:else}
						Empezar Importación
					{/if}
				</Button>
			{:else}
				<Button 
					onclick={() => (open = false)}
					class="bg-taupe text-parchment hover:bg-taupe/90 w-full"
				>
					Cerrar
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
