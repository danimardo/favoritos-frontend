<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { configuracion } from '$lib/stores/configuracion';
	import { CheckCircle2, XCircle, Loader2, LogIn, UserPlus } from 'lucide-svelte';
	import { goto } from '$app/navigation';

	let mode = $state<'login' | 'register'>('login');
	let apiUrl = $state($configuracion.apiUrl || 'http://localhost:3004/api/v1');
	let email = $state('');
	let password = $state('');
	let name = $state('');
	let loading = $state(false);
	let errorMessage = $state('');
	let successMessage = $state('');

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
			
			// Redirect to home after 1s
			setTimeout(() => {
				goto('/');
			}, 1000);
		} catch (e) {
			errorMessage = e instanceof Error ? e.message : String(e);
		} finally {
			loading = false;
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
		<div class="space-y-2">
			<Label for="apiUrl" class="text-taupe font-semibold">API URL</Label>
			<Input
				id="apiUrl"
				placeholder="http://localhost:3004/api/v1"
				bind:value={apiUrl}
				required
				class="bg-white border-silver focus:border-taupe"
			/>
		</div>

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

	<div class="pt-4 border-t border-silver/30 text-center">
		<p class="text-[10px] text-grey-olive">
			Tu API Key se almacenará de forma segura en tu sistema local.
		</p>
	</div>
</div>
