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
	<AlertDialog.Content class="bg-white border-2 border-red-100 shadow-2xl z-[100] opacity-100">
		<AlertDialog.Header>
			<AlertDialog.Title class="text-taupe font-bold text-xl">{title}</AlertDialog.Title>
			<AlertDialog.Description class="text-greyOlive">
				{description}
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer class="gap-2">
			<AlertDialog.Cancel 
				onclick={() => open = false}
				class="border-silver text-taupe hover:bg-silver/10"
			>
				Cancelar
			</AlertDialog.Cancel>
			<AlertDialog.Action 
				onclick={handleConfirm} 
				class="bg-red-600 text-white hover:bg-red-700 font-bold shadow-md px-6"
			>
				Eliminar Definitivamente
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
