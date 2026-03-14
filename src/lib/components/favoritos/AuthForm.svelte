<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { configuracion } from '$lib/stores/configuracion';
	import { CheckCircle2, XCircle, Loader2, LogIn, UserPlus, Settings, Save, Info } from 'lucide-svelte';
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/core';
	import * as Select from "$lib/components/ui/select";

	let mode = $state<'login' | 'register'>('login');
	let apiUrl = $state('http://localhost:3004/api/v1');
	let email = $state('');
	let password = $state('');
	let name = $state('');
	let loading = $state(false);
	let errorMessage = $state('');
	let successMessage = $state('');

	// Panel de ajustes de conexión
	let showSettings = $state(false);
	let apiKey = $state('');
	let logLevel = $state('info');
	let savingSettings = $state(false);
	let settingsSaved = $state(false);

	// Sincronizar valores del store cuando se cargue la configuración persistida
	let synced = $state(false);
	$effect(() => {
		if ($configuracion.ready && !synced) {
			apiUrl = $configuracion.apiUrl || 'http://localhost:3004/api/v1';
			logLevel = $configuracion.logLevel || 'info';
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

	async function handleSubmit() {
		loading = true;
		errorMessage = '';
		successMessage = '';

		try {
			if (mode === 'login') {
				await configuracion.login(apiUrl, email, password);
				successMessage = 'Sesión iniciada correctamente';
			} else {
				await configuracion.register(apiUrl, email, password, name || undefined);
				successMessage = 'Usuario registrado e inicio de sesión completado';
			}

			setTimeout(() => {
				goto('/');
			}, 1000);
		} catch (e) {
			errorMessage = e instanceof Error ? e.message : String(e);
		} finally {
			loading = false;
		}
	}

	async function handleSaveSettings() {
		savingSettings = true;
		settingsSaved = false;
		try {
			await invoke('config_update_settings', { apiUrl, apiKey: apiKey.trim() || null, logLevel });
			settingsSaved = true;
			setTimeout(() => { settingsSaved = false; }, 3000);
		} catch (error) {
			console.error('Error al guardar ajustes:', error);
		} finally {
			savingSettings = false;
		}
	}
</script>

<div class="space-y-6 p-6 bg-parchment rounded-xl border-2 border-taupe/10 shadow-sm max-w-md mx-auto">
	<div class="flex flex-col items-center gap-2 mb-4">
		<div class="p-3 bg-taupe rounded-full text-parchment">
			{#if mode === 'login'}
				<LogIn class="w-6 h-6" />
			{:else}
				<UserPlus class="w-6 h-6" />
			{/if}
		</div>
		<h2 class="text-2xl font-bold text-taupe">
			{mode === 'login' ? 'Iniciar Sesión' : 'Crear Cuenta'}
		</h2>
		<p class="text-grey-olive text-sm text-center">
			{mode === 'login' ? 'Accede a tus favoritos desde cualquier lugar' : 'Únete para empezar a organizar tus enlaces'}
		</p>
	</div>

	<div class="flex p-1 bg-silver/20 rounded-lg mb-6">
		<button
			class="flex-1 py-2 text-sm font-medium rounded-md transition-all {mode === 'login' ? 'bg-white text-taupe shadow-sm' : 'text-grey-olive hover:text-taupe'}"
			onclick={() => (mode = 'login')}
		>
			Login
		</button>
		<button
			class="flex-1 py-2 text-sm font-medium rounded-md transition-all {mode === 'register' ? 'bg-white text-taupe shadow-sm' : 'text-grey-olive hover:text-taupe'}"
			onclick={() => (mode = 'register')}
		>
			Registro
		</button>
	</div>

	<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-4">
		{#if mode === 'register'}
			<div class="space-y-2">
				<Label for="name" class="text-taupe font-semibold">Nombre (Opcional)</Label>
				<Input
					id="name"
					placeholder="Tu nombre"
					bind:value={name}
					class="bg-white border-silver focus:border-taupe"
				/>
			</div>
		{/if}

		<div class="space-y-2">
			<Label for="email" class="text-taupe font-semibold">Email</Label>
			<Input
				id="email"
				type="email"
				placeholder="usuario@ejemplo.com"
				bind:value={email}
				required
				class="bg-white border-silver focus:border-taupe"
			/>
		</div>

		<div class="space-y-2">
			<Label for="password" class="text-taupe font-semibold">Contraseña</Label>
			<Input
				id="password"
				type="password"
				placeholder="••••••••"
				bind:value={password}
				required
				minlength={8}
				class="bg-white border-silver focus:border-taupe"
			/>
		</div>

		{#if errorMessage}
			<div class="p-3 bg-red-50 text-red-600 text-sm rounded-lg flex items-center gap-2 border border-red-100">
				<XCircle class="w-4 h-4 flex-shrink-0" />
				<span>{errorMessage}</span>
			</div>
		{/if}

		{#if successMessage}
			<div class="p-3 bg-green-50 text-green-600 text-sm rounded-lg flex items-center gap-2 border border-green-100">
				<CheckCircle2 class="w-4 h-4 flex-shrink-0" />
				<span>{successMessage}</span>
			</div>
		{/if}

		<Button
			type="submit"
			class="w-full bg-taupe hover:bg-taupe/90 text-parchment h-11 text-lg font-bold shadow-md hover:shadow-lg transition-all"
			disabled={loading}
		>
			{#if loading}
				<Loader2 class="w-5 h-5 mr-2 animate-spin" />
				Procesando...
			{:else}
				{mode === 'login' ? 'Entrar' : 'Registrarse'}
			{/if}
		</Button>
	</form>

	<!-- Separador con botón de ajustes -->
	<div class="pt-4 border-t border-silver/30">
		<button
			type="button"
			class="w-full flex items-center justify-center gap-2 text-xs text-grey-olive hover:text-taupe transition-colors cursor-pointer py-1"
			onclick={() => (showSettings = !showSettings)}
		>
			<Settings class="w-3.5 h-3.5" />
			<span>Ajustes de conexión</span>
		</button>

		{#if showSettings}
			<div class="mt-4 space-y-4 p-4 bg-white/60 rounded-lg border border-silver/40">
				<div class="space-y-2">
					<Label for="settingsApiUrl" class="text-taupe font-semibold text-sm flex items-center gap-1.5">
						API URL
					</Label>
					<Input
						id="settingsApiUrl"
						placeholder="http://localhost:3004/api/v1"
						bind:value={apiUrl}
						class="bg-white border-silver focus:border-taupe font-mono text-sm"
					/>
				</div>

				<div class="space-y-2">
					<Label for="settingsApiKey" class="text-taupe font-semibold text-sm flex items-center gap-1.5">
						API Key
					</Label>
					<Input
						id="settingsApiKey"
						type="password"
						placeholder="Dejar vacío para mantener la actual"
						bind:value={apiKey}
						class="bg-white border-silver focus:border-taupe font-mono text-sm"
					/>
				</div>

				<div class="space-y-2">
					<Label class="text-taupe font-semibold text-sm flex items-center gap-1.5">
						Nivel de Log
					</Label>
					<Select.Root type="single" bind:value={logLevel}>
						<Select.Trigger class="w-full bg-white border-silver text-taupe text-sm">
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
				</div>

				<div class="flex items-start gap-2 p-2 bg-blue-50/50 rounded-md border border-blue-100">
					<Info class="w-3.5 h-3.5 text-blue-400 mt-0.5 flex-shrink-0" />
					<p class="text-[10px] text-blue-600 leading-relaxed">
						Modifica estos ajustes si ha cambiado la dirección de tu servidor, la API Key o necesitas ajustar la verbosidad del log.
					</p>
				</div>

				<div class="flex items-center justify-between">
					{#if settingsSaved}
						<span class="text-green-600 text-xs font-medium flex items-center gap-1">
							<CheckCircle2 class="w-3.5 h-3.5" />
							Guardado
						</span>
					{:else}
						<span></span>
					{/if}
					<Button
						type="button"
						variant="outline"
						size="sm"
						class="text-taupe border-taupe/30 hover:bg-taupe/5 gap-1.5 font-medium"
						onclick={handleSaveSettings}
						disabled={savingSettings}
					>
						{#if savingSettings}
							<Loader2 class="w-3.5 h-3.5 animate-spin" />
							Guardando...
						{:else}
							<Save class="w-3.5 h-3.5" />
							Guardar ajustes
						{/if}
					</Button>
				</div>
			</div>
		{/if}
	</div>
</div>
