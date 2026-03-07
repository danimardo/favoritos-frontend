<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Button } from '$lib/components/ui/button';

	let { open = $bindable(false), onConfirm, title = "Eliminar favorito", description = "¿Estás seguro de que deseas eliminar este favorito? Esta acción no se puede deshacer." } = $props();

	function handleConfirm() {
		onConfirm();
		open = false;
	}
</script>

<AlertDialog.Root bind:open>
	<AlertDialog.Content class="bg-parchment">
		<AlertDialog.Header>
			<AlertDialog.Title>{title}</AlertDialog.Title>
			<AlertDialog.Description>
				{description}
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel onclick={() => open = false}>Cancelar</AlertDialog.Cancel>
			<AlertDialog.Action onclick={handleConfirm} class="bg-red-600 text-white hover:bg-red-700">
				Eliminar
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
