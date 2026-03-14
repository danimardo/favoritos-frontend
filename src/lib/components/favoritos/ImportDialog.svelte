<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import { invoke } from '@tauri-apps/api/core';
	import { tauriNotifications } from '$lib/services/tauri';
	import { Globe, Upload, CheckCircle2, AlertCircle, Loader2, Sparkles } from 'lucide-svelte';
	import { favoritos } from '$lib/stores/favoritos';

	let { open = $bindable(false) } = $props();

	let fileInput = $state<HTMLInputElement | null>(null);
	let file = $state<File | null>(null);
	let loading = $state(false);
	let iaEnabled = $state(false);
	let result = $state<{ imported: number; updated: number; skipped: number; total: number; errors: string[] } | null>(null);
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
			const arrayBuffer = await file.arrayBuffer();
			const fileBytes = Array.from(new Uint8Array(arrayBuffer));

			console.log(`[Import] Enviando archivo "${file.name}" (${fileBytes.length} bytes, ia: ${iaEnabled})`);

			const importResult = await invoke('import_favorites', {
				fileBytes,
				filename: file.name,
				ia: iaEnabled
			}) as any;
			result = importResult;

			console.log(`[Import] Resultado: ${result?.imported ?? 0} importados, ${result?.updated ?? 0} actualizados, ${result?.skipped ?? 0} omitidos, ${result?.total ?? 0} total`);
			if (result?.errors?.length) {
				console.warn('[Import] Detalles:');
				for (const err of result.errors) {
					console.warn(`  - ${err}`);
				}
			}

			if (result && (result.imported > 0 || result.updated > 0)) {
				await favoritos.fetchAll();
				const parts = [];
				if (result.imported > 0) parts.push(`${result.imported} importados`);
				if (result.updated > 0) parts.push(`${result.updated} actualizados`);
				await tauriNotifications.showSuccess(`Importación completada: ${parts.join(', ')}.`);
			} else if (result && result.imported === 0 && result.updated === 0) {
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
		iaEnabled = false;
		if (fileInput) fileInput.value = '';
	}
</script>

<Dialog.Root bind:open onOpenChange={(o) => !o && reset()}>
	<Dialog.Content class="sm:max-w-[425px] bg-white border-2 border-taupe/20 shadow-2xl z-[100] opacity-100">
		<Dialog.Header>
			<Dialog.Title class="text-taupe flex items-center gap-2 font-bold text-xl">
				<Globe class="h-6 w-6" />
				Importar Favoritos
			</Dialog.Title>
			<Dialog.Description class="text-greyOlive">
				Sube un archivo HTML de marcadores exportado desde tu navegador (Firefox, Chrome, etc.).
			</Dialog.Description>
		</Dialog.Header>

		<div class="grid gap-4 py-4">
			{#if !result}
				<div class="grid w-full items-center gap-1.5">
					<Label for="html-file" class="text-taupe font-semibold">Archivo HTML de marcadores</Label>
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
							Solo archivos .html o .htm (formato Netscape Bookmark)
						</p>
						<input
							id="html-file"
							type="file"
							accept=".html,.htm"
							class="hidden"
							bind:this={fileInput}
							onchange={handleFileChange}
						/>
					</div>
				</div>

				<!-- Toggle de IA -->
				<div class="flex items-center justify-between p-3 bg-silver/5 rounded-lg border border-silver/30">
					<div class="flex-1 mr-3">
						<p class="text-sm font-semibold text-taupe flex items-center gap-1.5">
							<Sparkles class="h-4 w-4 text-amber-500" />
							Enriquecimiento con IA
						</p>
						<p class="text-[10px] text-grey-olive mt-0.5 leading-relaxed">
							Analiza cada marcador con IA para completar automáticamente título, descripción y etiquetas. El proceso es más lento pero mejora la calidad de los datos.
						</p>
					</div>
					<button
						type="button"
						role="switch"
						aria-checked={iaEnabled}
						class="relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-taupe focus-visible:ring-offset-2 {iaEnabled ? 'bg-amber-500' : 'bg-silver'}"
						onclick={() => iaEnabled = !iaEnabled}
					>
						<span
							class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow-lg ring-0 transition duration-200 ease-in-out {iaEnabled ? 'translate-x-5' : 'translate-x-0'}"
						></span>
					</button>
				</div>

				{#if error}
					<div class="p-3 bg-red-50 text-red-600 text-xs rounded-lg border border-red-100 flex items-start gap-2 shadow-sm">
						<AlertCircle class="h-4 w-4 shrink-0 mt-0.5" />
						<span class="leading-tight font-medium">{error}</span>
					</div>
				{/if}
			{:else}
				<div class="bg-silver/10 rounded-xl p-5 space-y-4 border border-silver/30">
					<div class="flex items-center gap-2 text-green-600 font-bold">
						<CheckCircle2 class="h-6 w-6" />
						Importación finalizada
					</div>
					<div class="grid grid-cols-2 gap-3 text-sm">
						<div class="bg-white p-3 rounded-lg border border-silver/50 shadow-sm text-center">
							<p class="text-[10px] text-grey-olive uppercase tracking-wider font-bold">Importados</p>
							<p class="text-2xl font-black text-green-600">{result.imported}</p>
						</div>
						<div class="bg-white p-3 rounded-lg border border-silver/50 shadow-sm text-center">
							<p class="text-[10px] text-grey-olive uppercase tracking-wider font-bold">Actualizados</p>
							<p class="text-2xl font-black text-blue-500">{result.updated}</p>
						</div>
						<div class="bg-white p-3 rounded-lg border border-silver/50 shadow-sm text-center">
							<p class="text-[10px] text-grey-olive uppercase tracking-wider font-bold">Omitidos</p>
							<p class="text-2xl font-black text-red-400">{result.skipped}</p>
						</div>
						<div class="bg-white p-3 rounded-lg border border-silver/50 shadow-sm text-center">
							<p class="text-[10px] text-grey-olive uppercase tracking-wider font-bold">Total</p>
							<p class="text-2xl font-black text-taupe">{result.total}</p>
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
