<script lang="ts">
	import ConfigForm from '$lib/components/favoritos/ConfigForm.svelte';
	import ImportDialog from '$lib/components/favoritos/ImportDialog.svelte';
	import { Separator } from '$lib/components/ui/separator';
	import { Button } from '$lib/components/ui/button';
	import { FileJson, ArrowLeft } from 'lucide-svelte';
	import { isConfigured } from '$lib/stores/configuracion';
	import { goto } from '$app/navigation';

	let importOpen = $state(false);
</script>

<div class="max-w-2xl mx-auto space-y-6">
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
		<h2 class="text-xl font-semibold text-taupe">Conexión API</h2>
		<ConfigForm />
	</div>

	<Separator />

	<div class="space-y-4">
		<h2 class="text-xl font-semibold text-taupe">Importación de Datos</h2>
		<p class="text-sm text-grey-olive">
			Importa tus favoritos desde un archivo JSON compatible.
		</p>
		<Button 
			variant="outline" 
			class="border-taupe text-taupe hover:bg-taupe/10"
			onclick={() => (importOpen = true)}
		>
			<FileJson class="mr-2 h-4 w-4" />
			Importar JSON
		</Button>
		<ImportDialog bind:open={importOpen} />
	</div>

	<div class="pt-8">
		<h2 class="text-xl font-semibold text-taupe">Información del Sistema</h2>
		<div class="mt-4 p-4 rounded-lg bg-silver/20 border border-silver text-sm text-greyOlive">
			<p>Favoritos Desktop v1.0.0</p>
			<p>Plataforma: Tauri v2</p>
		</div>
	</div>
</div>
