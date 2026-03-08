<script lang="ts">
	import ConfigForm from '$lib/components/favoritos/ConfigForm.svelte';
	import ImportDialog from '$lib/components/favoritos/ImportDialog.svelte';
	import { Separator } from '$lib/components/ui/separator';
	import { Button } from '$lib/components/ui/button';
	import { FileJson, ArrowLeft, Download, Upload, Loader2 } from 'lucide-svelte';
	import { isConfigured } from '$lib/stores/configuracion';
	import { favoritos } from '$lib/stores/favoritos';
	import { goto } from '$app/navigation';

	let importOpen = $state(false);
	let exporting = $state(false);

	async function handleExport() {
		exporting = true;
		try {
			await favoritos.exportAll();
		} finally {
			exporting = false;
		}
	}
</script>

<div class="max-w-2xl mx-auto space-y-6 pb-12">
	<div class="flex items-center justify-between">
		<div class="flex items-center gap-4">
			{#if $isConfigured}
				<Button variant="ghost" size="icon" onclick={() => goto('/')} class="text-taupe hover:bg-taupe/10">
					<ArrowLeft class="h-5 w-5" />
				</Button>
			{/if}
			<div>
				<h1 class="text-3xl font-bold text-taupe">Configuración</h1>
				<p class="text-greyOlive">Gestiona la conexión con tu API y otras preferencias.</p>
			</div>
		</div>
	</div>

	<Separator />

	<div class="space-y-4">
		<h2 class="text-xl font-semibold text-taupe">Ajustes y Perfil</h2>
		<ConfigForm />
	</div>

	{#if $isConfigured}
		<Separator />

		<div class="space-y-4">
			<h2 class="text-xl font-semibold text-taupe">Gestión de Datos</h2>
			<p class="text-sm text-grey-olive">
				Exporta tu colección completa o importa datos desde otros equipos.
			</p>
			<div class="flex flex-wrap gap-3">
				<Button 
					variant="outline" 
					class="border-taupe text-taupe hover:bg-taupe/5 font-bold"
					onclick={() => (importOpen = true)}
				>
					<Upload class="mr-2 h-4 w-4" />
					Importar JSON
				</Button>

				<Button 
					variant="outline" 
					class="border-taupe text-taupe hover:bg-taupe/5 font-bold"
					onclick={handleExport}
					disabled={exporting}
				>
					{#if exporting}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
						Exportando...
					{:else}
						<Download class="mr-2 h-4 w-4" />
						Exportar Todo
					{/if}
				</Button>
			</div>
			<ImportDialog bind:open={importOpen} />
		</div>
	{/if}

	<div class="pt-8">
		<h2 class="text-xl font-semibold text-taupe text-sm">Información del Sistema</h2>
		<div class="mt-4 p-4 rounded-lg bg-silver/10 border border-silver/30 text-[10px] text-grey-olive font-mono">
			<p>Favoritos Desktop v0.3.0</p>
			<p>Plataforma: Tauri v2 (Rust + SvelteKit)</p>
			<p>Data Model: Unified JSON v1</p>
		</div>
	</div>
</div>
