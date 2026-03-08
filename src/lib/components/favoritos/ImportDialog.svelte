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
		result = null;

		try {
			const text = await file.text();
			
			// Client-side validation for better feedback
			let data;
			try {
				data = JSON.parse(text);
			} catch (e) {
				throw new Error('El archivo no es un JSON válido.');
			}

			if (!Array.isArray(data) && (!data.bookmarks || !Array.isArray(data.bookmarks))) {
				throw new Error('Formato inválido. El JSON debe ser un array o un objeto con la propiedad "bookmarks".');
			}

			// Pre-process if it's an array to match the bulk API format { bookmarks: [...] }
			const payload = Array.isArray(data) ? { bookmarks: data } : data;
			const payloadString = JSON.stringify(payload);

			console.log('[Import] Enviando datos masivos...');
			const importResult = await invoke('import_favorites', { data: payloadString }) as any;
			result = importResult;
			
			if (result && result.imported > 0) {
				await favoritos.fetchAll(); // Refresh list
				await tauriNotifications.showSuccess(`Importación completada: ${result.imported} favoritos añadidos.`);
			} else if (result && result.imported === 0) {
				error = "No se importó ningún marcador. Revisa si ya existen o si hay errores.";
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Error al importar el archivo.';
			console.error('[Import Error]', e);
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
	<Dialog.Content class="sm:max-w-[425px] bg-white border-2 border-taupe/20 shadow-2xl z-[100] opacity-100">
		<Dialog.Header>
			<Dialog.Title class="text-taupe flex items-center gap-2 font-bold text-xl">
				<FileJson class="h-6 w-6" />
				Importar Favoritos
			</Dialog.Title>
			<Dialog.Description class="text-greyOlive">
				Sube un archivo JSON con tu colección de favoritos para importarlos masivamente.
			</Dialog.Description>
		</Dialog.Header>

		<div class="grid gap-4 py-4">
			{#if !result}
				<div class="grid w-full items-center gap-1.5">
					<Label for="json-file" class="text-taupe font-semibold">Archivo JSON</Label>
					<div 
						class="flex flex-col items-center justify-center border-2 border-dashed border-silver rounded-xl p-8 bg-silver/5 hover:bg-taupe/5 hover:border-taupe/50 transition-all cursor-pointer group"
						onclick={() => fileInput?.click()}
						onkeydown={(e) => e.key === 'Enter' && fileInput?.click()}
						role="button"
						tabindex="0"
					>
						<div class="p-3 bg-white rounded-full shadow-sm mb-3 group-hover:scale-110 transition-transform">
							<Upload class="h-8 w-8 text-taupe" />
						</div>
						<p class="text-sm text-taupe font-medium text-center">
							{file ? file.name : 'Haz clic para seleccionar archivo'}
						</p>
						<p class="text-[10px] text-grey-olive mt-1">
							Solo se admiten archivos .json
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
						<div class="p-3 bg-red-50 text-red-600 text-xs rounded-lg border border-red-100 mt-2 flex items-start gap-2 shadow-sm">
							<AlertCircle class="h-4 w-4 shrink-0 mt-0.5" />
							<span class="leading-tight font-medium">{error}</span>
						</div>
					{/if}
				</div>
			{:else}
				<div class="bg-silver/10 rounded-xl p-5 space-y-4 border border-silver/30">
					<div class="flex items-center gap-2 text-green-600 font-bold">
						<CheckCircle2 class="h-6 w-6" />
						Importación finalizada
					</div>
					<div class="grid grid-cols-2 gap-4 text-sm">
						<div class="bg-white p-3 rounded-lg border border-silver/50 shadow-sm text-center">
							<p class="text-[10px] text-grey-olive uppercase tracking-wider font-bold">Importados</p>
							<p class="text-3xl font-black text-taupe">{result.imported}</p>
						</div>
						<div class="bg-white p-3 rounded-lg border border-silver/50 shadow-sm text-center">
							<p class="text-[10px] text-grey-olive uppercase tracking-wider font-bold">Omitidos</p>
							<p class="text-3xl font-black text-red-400">{result.skipped}</p>
						</div>
					</div>
					{#if result.errors.length > 0}
						<div class="bg-white/50 rounded-lg p-3 border border-red-100 shadow-inner">
							<p class="text-xs font-bold text-red-600 mb-2">Mensajes del servidor:</p>
							<ul class="text-[10px] text-red-500 list-disc pl-4 space-y-1 max-h-32 overflow-y-auto custom-scrollbar">
								{#each result.errors as err}
									<li>{err}</li>
								{/each}
							</ul>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<Dialog.Footer class="gap-2">
			{#if !result}
				<Button 
					variant="ghost" 
					onclick={() => (open = false)}
					disabled={loading}
					class="text-grey-olive hover:text-taupe"
				>
					Cancelar
				</Button>
				<Button 
					onclick={startImport} 
					disabled={!file || loading}
					class="bg-taupe text-parchment hover:bg-taupe/90 font-bold shadow-md px-6 min-w-[140px]"
				>
					{#if loading}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
						Importando...
					{:else}
						Importar ahora
					{/if}
				</Button>
			{:else}
				<Button 
					onclick={() => { open = false; reset(); }}
					class="bg-taupe text-parchment hover:bg-taupe/90 w-full font-bold h-11 shadow-md"
				>
					Finalizar
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 4px;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: #C0C0C0;
		border-radius: 4px;
	}
</style>
