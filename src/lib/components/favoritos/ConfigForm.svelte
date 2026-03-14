<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { configuracion, currentUser } from '$lib/stores/configuracion';
	import { LogOut, User, Mail, Shield, Key, Settings2, Info, Loader2, CheckCircle2, Save } from 'lucide-svelte';
	import * as Select from "$lib/components/ui/select";
	import AuthForm from './AuthForm.svelte';

	let apiUrl = $state($configuracion.apiUrl || 'http://localhost:3004/api/v1');
	let apiKey = $state('');
	let logLevel = $state($configuracion.logLevel || 'info');
	let saving = $state(false);
	let saveSuccess = $state(false);

	// Re-sincronizar cuando el store se actualice (ej. si init() aún no había completado al montar)
	let synced = $state(false);
	$effect(() => {
		if ($configuracion.ready && !synced) {
			if ($configuracion.apiUrl) {
				apiUrl = $configuracion.apiUrl;
			}
			if ($configuracion.logLevel) {
				logLevel = $configuracion.logLevel;
			}
			synced = true;
		}
	});

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

	async function handleLogout() {
		if (confirm('¿Estás seguro de que quieres cerrar sesión?')) {
			await configuracion.logout();
		}
	}

	async function handleGlobalSave() {
		saving = true;
		saveSuccess = false;
		try {
			await configuracion.saveSettings(apiUrl, logLevel, apiKey.trim() || undefined);
			apiKey = '';
			saveSuccess = true;
			setTimeout(() => {
				saveSuccess = false;
			}, 3000);
		} catch (error) {
			console.error('Error al guardar cambios globales:', error);
		} finally {
			saving = false;
		}
	}
</script>

{#if !$configuracion.user}
	<div class="py-8">
		<AuthForm />
	</div>
{:else}
	<div class="space-y-6 max-w-2xl mx-auto pb-20 relative">
		<!-- Perfil de Usuario -->
		<div class="bg-white rounded-xl border border-silver/50 overflow-hidden shadow-sm">
			<div class="bg-taupe p-4 text-parchment flex items-center justify-between">
				<div class="flex items-center gap-3">
					<div class="p-2 bg-white/20 rounded-lg">
						<User class="w-6 h-6" />
					</div>
					<div>
						<h3 class="font-bold text-lg">Perfil de Usuario</h3>
						<p class="text-xs text-parchment/70">Gestión de cuenta y servidor</p>
					</div>
				</div>
				<Button 
					variant="ghost" 
					size="sm"
					class="text-parchment hover:bg-white/10 hover:text-white gap-2 font-medium" 
					onclick={handleLogout}
				>
					<LogOut class="w-4 h-4" />
					Cerrar Sesión
				</Button>
			</div>
			
			<div class="p-6 space-y-6">
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="space-y-2">
						<Label class="text-taupe font-semibold flex items-center gap-2">
							<Mail class="w-4 h-4" /> Email
						</Label>
						<Input value={$currentUser?.email} disabled class="bg-silver/5 border-silver/30 text-grey-olive cursor-not-allowed" />
					</div>
					<div class="space-y-2">
						<Label class="text-taupe font-semibold flex items-center gap-2">
							<User class="w-4 h-4" /> Nombre
						</Label>
						<Input value={$currentUser?.name || 'Invitado'} disabled class="bg-silver/5 border-silver/30 text-grey-olive cursor-not-allowed" />
					</div>
				</div>

				<div class="space-y-2 pt-2">
					<Label for="apiUrl" class="text-taupe font-semibold flex items-center gap-2">
						<Shield class="w-4 h-4" /> Dirección del API (Servidor)
					</Label>
					<Input
						id="apiUrl"
						bind:value={apiUrl}
						placeholder="http://tu-servidor:3004/api/v1"
						class="bg-white border-silver focus:ring-taupe focus:border-taupe font-mono text-sm"
					/>
					<p class="text-[10px] text-grey-olive italic">
						Modifica esta URL solo si has cambiado la dirección de tu servidor Favoritos.
					</p>
				</div>

				<div class="space-y-2 pt-2">
					<Label for="apiKey" class="text-taupe font-semibold flex items-center gap-2">
						<Key class="w-4 h-4" /> API Key
					</Label>
					<Input
						id="apiKey"
						type="password"
						bind:value={apiKey}
						placeholder="Dejar vacío para mantener la actual"
						class="bg-white border-silver focus:ring-taupe focus:border-taupe font-mono text-sm"
					/>
					<p class="text-[10px] text-grey-olive italic">
						Solo rellena este campo si necesitas cambiar tu API Key. Déjalo vacío para conservar la actual.
					</p>
				</div>
			</div>
		</div>

		<!-- Ajustes del Sistema -->
		<div class="bg-white rounded-xl border border-silver/50 overflow-hidden shadow-sm">
			<div class="bg-silver/10 p-4 text-taupe flex items-center gap-3 border-b border-silver/50">
				<div class="p-2 bg-taupe/10 rounded-lg">
					<Settings2 class="w-6 h-6" />
				</div>
				<div>
					<h3 class="font-bold text-lg">Ajustes del Sistema</h3>
					<p class="text-xs text-grey-olive">Configuración técnica de la aplicación</p>
				</div>
			</div>

			<div class="p-6 space-y-6">
				<div class="space-y-3">
					<Label class="text-taupe font-semibold flex items-center gap-2">
						<Info class="w-4 h-4" /> Nivel de Log (Verbosity)
					</Label>
					<Select.Root type="single" bind:value={logLevel}>
						<Select.Trigger class="w-full bg-white border-silver text-taupe font-medium">
							{selectedLogLevelLabel}
						</Select.Trigger>
						<Select.Content class="bg-white border-silver">
							{#each logLevels as level}
								<Select.Item value={level.value} label={level.label} class="text-taupe cursor-pointer hover:bg-silver/10">
									{level.label}
								</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
					<div class="flex items-start gap-2 p-3 bg-blue-50/50 rounded-lg border border-blue-100">
						<Info class="w-4 h-4 text-blue-400 mt-0.5" />
						<p class="text-[11px] text-blue-600 leading-relaxed">
							El nivel de log determina qué información técnica se registra. Los cambios se aplicarán completamente al reiniciar Favoritos Desktop.
						</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Botón de Guardado Global (Blanco / Outline) -->
		<div class="flex items-center justify-between pt-4">
			{#if saveSuccess}
				<div class="text-green-600 text-sm font-medium flex items-center gap-2 animate-in fade-in slide-in-from-left-4">
					<CheckCircle2 class="w-5 h-5" />
					¡Cambios guardados con éxito!
				</div>
			{:else}
				<div></div>
			{/if}
			
			<Button 
				variant="outline"
				class="bg-white hover:bg-taupe/5 text-taupe border-taupe/30 font-bold px-8 py-6 h-auto text-lg shadow-sm active:scale-95 transition-all gap-2" 
				onclick={handleGlobalSave}
				disabled={saving}
			>
				{#if saving}
					<Loader2 class="w-5 h-5 animate-spin" />
					Guardando...
				{:else}
					<Save class="w-5 h-5" />
					Guardar Cambios
				{/if}
			</Button>
		</div>
	</div>
{/if}
