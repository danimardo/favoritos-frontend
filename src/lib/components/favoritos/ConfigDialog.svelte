<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import ConfigForm from './ConfigForm.svelte';
	import { isConfigured } from '$lib/stores/configuracion';

	export let open = false;
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="sm:max-w-[425px] bg-parchment">
		<Dialog.Header>
			<Dialog.Title>Configuración de API</Dialog.Title>
			<Dialog.Description>
				Ingresa los detalles de tu API de Favoritos para comenzar.
			</Dialog.Description>
		</Dialog.Header>
		<ConfigForm />
	</Dialog.Content>
</Dialog.Root>
