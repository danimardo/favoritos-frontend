<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { configuracion } from '$lib/stores/configuracion';
	import { CheckCircle2, XCircle, Loader2 } from 'lucide-svelte';
	import { goto } from '$app/navigation';

	import * as Select from "$lib/components/ui/select";

	let apiUrl = $state($configuracion.apiUrl || '');
	let apiKey = $state(''); // Never show the key back
	let logLevel = $state($configuracion.logLevel || 'info');
	let validating = $state(false);
	let valid = $state<boolean | null>(null);
	let errorMessage = $state('');
	let saveSuccess = $state(false);

	const logLevels = [
		{ value: "error", label: "Error (Mínimo)" },
		{ value: "warn", label: "Warning" },
		{ value: "info", label: "Info (Normal)" },
		{ value: "debug", label: "Debug (Verboso)" },
		{ value: "trace", label: "Trace (Máximo)" },
	];

	const selectedLogLevelLabel = $derived(
		logLevels.find((l) => l.value === logLevel)?.label ?? "Seleccionar nivel"
	);

	async function handleValidate() {
		validating = true;
		valid = null;
		saveSuccess = false;
		errorMessage = '';
		try {
			valid = await configuracion.validate(apiUrl, apiKey);
			if (valid !== true) {
				if (typeof valid === 'string') {
					errorMessage = valid;
					valid = false;
				} else {
					errorMessage = 'Conexión fallida. Verifica los datos.';
					valid = false;
				}
			}
		} catch (e) {
			valid = false;
			errorMessage = e instanceof Error ? e.message : String(e);
		} finally {
			validating = false;
		}
	}

	async function handleSave() {
		try {
			errorMessage = '';
			await configuracion.save(apiUrl, apiKey, logLevel);
			saveSuccess = true;
			valid = null; // Hide validation success msg
			
			// Redirect to home after 1.5s
			setTimeout(() => {
				goto('/');
			}, 1500);
		} catch (e) {
			saveSuccess = false;
			errorMessage = e instanceof Error ? e.message : String(e);
		}
	}
</script>

<div class="space-y-4 p-4 bg-white rounded-lg border border-silver/50">
	<div class="space-y-2">
		<Label for="apiUrl">API URL</Label>
		<Input
			id="apiUrl"
			placeholder="https://api.favoritos.com"
			bind:value={apiUrl}
			disabled={validating}
		/>
	</div>

	<div class="space-y-2">
		<Label for="apiKey">API Key</Label>
		<Input
			id="apiKey"
			type="password"
			placeholder="sk-..."
			bind:value={apiKey}
			disabled={validating}
		/>
	</div>

	<div class="space-y-2">
		<Label style="color: #483C32;">Nivel de Log (Verbosity)</Label>
		<Select.Root type="single" bind:value={logLevel}>
			<Select.Trigger class="w-full" style="background-color: white; border: 1px solid #C0C0C0; color: #483C32;">
				{selectedLogLevelLabel}
			</Select.Trigger>
			<Select.Content style="background-color: white; border: 1px solid #C0C0C0; color: #483C32;">
				{#each logLevels as level}
					<Select.Item value={level.value} label={level.label} style="color: #483C32; cursor: pointer;">
						{level.label}
					</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
		<p class="text-[10px]" style="color: #A9A9A1;">Requiere reiniciar la aplicación para aplicar cambios de log.</p>
	</div>

	{#if errorMessage}
		<div class="text-red-600 text-sm flex items-center gap-2">
			<XCircle class="w-4 h-4" />
			{errorMessage}
		</div>
	{/if}

	{#if valid === true}
		<div class="text-green-600 text-sm flex items-center gap-2">
			<CheckCircle2 class="w-4 h-4" />
			Conexión validada con éxito
		</div>
	{/if}

	{#if saveSuccess}
		<div class="text-green-600 text-sm flex items-center gap-2">
			<CheckCircle2 class="w-4 h-4" />
			Configuración guardada correctamente
		</div>
	{/if}

	<div class="flex gap-2 pt-2">
		<Button variant="outline" class="flex-1" onclick={handleValidate} disabled={validating || !apiUrl || !apiKey}>
			{#if validating}
				<Loader2 class="w-4 h-4 mr-2 animate-spin" />
				Validando...
			{:else}
				Validar Conexión
			{/if}
		</Button>
		<Button class="flex-1 bg-taupe hover:bg-taupe/90 text-parchment" onclick={handleSave} disabled={validating || !apiUrl || !apiKey}>
			Guardar
		</Button>
	</div>
</div>
