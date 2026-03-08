<script lang="ts">
	import { onMount } from 'svelte';
	import { favoritos } from '$lib/stores/favoritos';
	import FavoritoGrid from '$lib/components/favoritos/FavoritoGrid.svelte';
	import FavoritoForm from '$lib/components/favoritos/FavoritoForm.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { isConfigured } from '$lib/stores/configuracion';

	let open = $state(false);
	let gridComponent = $state<any>(null);

	onMount(() => {
		if ($isConfigured) {
			favoritos.fetchAll();
		}

		window.addEventListener('keydown', handleKeydown);
		return () => window.removeEventListener('keydown', handleKeydown);
	});

	function handleKeydown(e: KeyboardEvent) {
		// Ctrl + N (Nuevo favorito)
		if (e.ctrlKey && e.key === 'n') {
			e.preventDefault();
			open = true;
		}
		// Ctrl + F (Buscar)
		if (e.ctrlKey && e.key === 'f') {
			e.preventDefault();
			if (gridComponent) gridComponent.focusSearch();
		}
	}

	function handleSave() {
		open = false;
	}
</script>

<div class="space-y-8">
	<div class="flex justify-between items-center">
		<div>
			<h1 class="text-4xl font-bold text-taupe">Mis Favoritos</h1>
			<p class="text-greyOlive mt-2">Gestiona tus enlaces importantes de forma segura.</p>
		</div>
		
		<Dialog.Root bind:open>
			<Dialog.Trigger>
				<Button class="bg-taupe text-parchment hover:bg-taupe/90 font-semibold shadow-md">
					<span class="mr-2">+</span> Nuevo Favorito
				</Button>
			</Dialog.Trigger>
			<Dialog.Content class="bg-white border-silver/50 shadow-2xl opacity-100">
				<Dialog.Header>
					<Dialog.Title class="text-taupe">Nuevo Favorito</Dialog.Title>
					<Dialog.Description class="text-greyOlive">
						Añade un nuevo enlace a tu colección.
					</Dialog.Description>
				</Dialog.Header>
				<FavoritoForm onSave={handleSave} />
			</Dialog.Content>
		</Dialog.Root>
	</div>

	<FavoritoGrid bind:this={gridComponent} />
</div>
