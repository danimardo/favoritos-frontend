<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Download, FileJson, Globe, Loader2 } from 'lucide-svelte';
	import { favoritos } from '$lib/stores/favoritos';

	let { open = $bindable(false) } = $props();

	let exporting = $state(false);

	async function handleExport(format: 'json' | 'html') {
		exporting = true;
		try {
			if (format === 'json') {
				await favoritos.exportAll();
			} else {
				await favoritos.exportHtml();
			}
			open = false;
		} finally {
			exporting = false;
		}
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="sm:max-w-[400px] bg-white border-2 border-taupe/20 shadow-2xl z-[100] opacity-100">
		<Dialog.Header>
			<Dialog.Title class="text-taupe flex items-center gap-2 font-bold text-xl">
				<Download class="h-6 w-6" />
				Exportar Favoritos
			</Dialog.Title>
			<Dialog.Description class="text-greyOlive">
				Elige el formato en el que quieres exportar tu colección de favoritos.
			</Dialog.Description>
		</Dialog.Header>

		<div class="grid gap-3 py-4">
			<button
				class="flex items-start gap-4 p-4 rounded-xl border-2 border-silver/30 bg-white hover:border-taupe/50 hover:bg-taupe/5 transition-all text-left group disabled:opacity-50 disabled:cursor-not-allowed"
				onclick={() => handleExport('json')}
				disabled={exporting}
			>
				<div class="p-2.5 bg-amber-50 rounded-lg border border-amber-200 group-hover:scale-110 transition-transform">
					<FileJson class="h-6 w-6 text-amber-600" />
				</div>
				<div class="flex-1">
					<p class="font-bold text-taupe text-sm">JSON</p>
					<p class="text-[11px] text-grey-olive mt-0.5 leading-relaxed">
						Formato propio de la aplicación. Ideal para copias de seguridad o migrar entre instancias de Favoritos Desktop.
					</p>
				</div>
			</button>

			<button
				class="flex items-start gap-4 p-4 rounded-xl border-2 border-silver/30 bg-white hover:border-taupe/50 hover:bg-taupe/5 transition-all text-left group disabled:opacity-50 disabled:cursor-not-allowed"
				onclick={() => handleExport('html')}
				disabled={exporting}
			>
				<div class="p-2.5 bg-blue-50 rounded-lg border border-blue-200 group-hover:scale-110 transition-transform">
					<Globe class="h-6 w-6 text-blue-600" />
				</div>
				<div class="flex-1">
					<p class="font-bold text-taupe text-sm">HTML de Marcadores</p>
					<p class="text-[11px] text-grey-olive mt-0.5 leading-relaxed">
						Formato Netscape Bookmark, compatible con Firefox, Chrome, Edge y otros navegadores.
					</p>
				</div>
			</button>
		</div>

		{#if exporting}
			<div class="flex items-center justify-center gap-2 py-2 text-sm text-taupe">
				<Loader2 class="h-4 w-4 animate-spin" />
				Exportando...
			</div>
		{/if}

		<Dialog.Footer>
			<Button
				variant="ghost"
				onclick={() => (open = false)}
				disabled={exporting}
				class="text-grey-olive hover:text-taupe w-full"
			>
				Cancelar
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
