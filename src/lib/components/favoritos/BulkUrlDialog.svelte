<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import { favoritos } from '$lib/stores/favoritos';
	import { Link, Loader2, CheckCircle2, AlertCircle, XCircle } from 'lucide-svelte';

	let { open = $bindable(false) } = $props();

	let urlText = $state('');
	let loading = $state(false);
	let results = $state<{ url: string; status: 'ok' | 'duplicate' | 'error'; message?: string }[]>([]);
	let done = $state(false);
	let currentIndex = $state(0);
	let totalUrls = $state(0);

	interface LineValidation {
		line: number;
		text: string;
		valid: boolean;
		error?: string;
	}

	function validateLines(text: string): LineValidation[] {
		return text
			.split('\n')
			.map((raw, i) => {
				const trimmed = raw.trim();
				if (trimmed.length === 0) return null; // Ignorar líneas vacías
				const lineNum = i + 1;

				if (!/^https?:\/\//i.test(trimmed)) {
					return { line: lineNum, text: trimmed, valid: false, error: 'Debe empezar por http:// o https://' };
				}
				try {
					const parsed = new URL(trimmed);
					if (!parsed.hostname.includes('.')) {
						return { line: lineNum, text: trimmed, valid: false, error: 'Dominio inválido' };
					}
				} catch {
					return { line: lineNum, text: trimmed, valid: false, error: 'Formato de URL inválido' };
				}
				return { line: lineNum, text: trimmed, valid: true };
			})
			.filter((v): v is LineValidation => v !== null);
	}

	let validations = $derived(validateLines(urlText));
	let validUrls = $derived(validations.filter((v) => v.valid));
	let invalidUrls = $derived(validations.filter((v) => !v.valid));
	let hasErrors = $derived(invalidUrls.length > 0);

	async function startBulkAdd() {
		if (hasErrors || validUrls.length === 0) return;

		loading = true;
		done = false;
		results = [];
		totalUrls = validUrls.length;
		currentIndex = 0;

		for (const entry of validUrls) {
			currentIndex++;
			try {
				await favoritos.create({ url: entry.text, title: '' });
				results = [...results, { url: entry.text, status: 'ok' }];
			} catch (e) {
				const errMsg = e instanceof Error ? e.message : String(e);
				if (errMsg.includes('409') || errMsg.includes('Conflict')) {
					results = [...results, { url: entry.text, status: 'duplicate', message: 'Ya existe' }];
				} else {
					results = [...results, { url: entry.text, status: 'error', message: errMsg }];
				}
			}
		}

		loading = false;
		done = true;

		const created = results.filter((r) => r.status === 'ok').length;
		if (created > 0) {
			await favoritos.fetchAll();
		}
	}

	function reset() {
		urlText = '';
		results = [];
		done = false;
		currentIndex = 0;
		totalUrls = 0;
	}
</script>

<Dialog.Root bind:open onOpenChange={(o) => !o && reset()}>
	<Dialog.Content class="sm:max-w-[500px] bg-white border-2 border-taupe/20 shadow-2xl z-[100] opacity-100">
		<Dialog.Header>
			<Dialog.Title class="text-taupe flex items-center gap-2 font-bold text-xl">
				<Link class="h-6 w-6" />
				Añadir URL sueltas
			</Dialog.Title>
			<Dialog.Description class="text-greyOlive">
				Introduce una URL por línea. Se creará un favorito para cada una.
			</Dialog.Description>
		</Dialog.Header>

		<div class="grid gap-4 py-4">
			{#if !done}
				<div class="space-y-2">
					<div class="flex justify-between items-center">
						<Label for="bulk-urls" class="text-taupe font-semibold">URLs</Label>
						{#if validations.length > 0}
							<span class="text-[10px] font-medium {hasErrors ? 'text-red-500' : 'text-greyOlive'}">
								{validUrls.length} válida{validUrls.length !== 1 ? 's' : ''}{#if hasErrors}, {invalidUrls.length} con error{/if}
							</span>
						{/if}
					</div>
					<textarea
						id="bulk-urls"
						bind:value={urlText}
						placeholder={"https://ejemplo.com\nhttps://otro-sitio.org\nhttps://github.com/user/repo"}
						disabled={loading}
						rows={8}
						class="flex w-full rounded-md border bg-white px-3 py-2 text-sm font-mono focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-taupe/20 focus-visible:border-taupe disabled:opacity-50 {hasErrors ? 'border-red-300' : 'border-silver'}"
					></textarea>
				</div>

				{#if hasErrors}
					<div class="p-3 bg-red-50 text-red-600 text-xs rounded-lg border border-red-100 space-y-1.5">
						<p class="font-bold flex items-center gap-1.5">
							<AlertCircle class="h-4 w-4 shrink-0" />
							Corrige las siguientes líneas antes de continuar:
						</p>
						<ul class="pl-5 space-y-0.5">
							{#each invalidUrls as inv}
								<li>
									<span class="font-semibold">Línea {inv.line}:</span>
									<span class="break-all">{inv.text}</span>
									— <span class="italic">{inv.error}</span>
								</li>
							{/each}
						</ul>
					</div>
				{/if}

				{#if loading}
					<div class="p-3 bg-blue-50 text-blue-700 text-xs rounded-lg border border-blue-200 flex items-center gap-2">
						<Loader2 class="h-4 w-4 animate-spin shrink-0" />
						<span>Procesando {currentIndex} de {totalUrls}...</span>
					</div>
				{/if}
			{:else}
				<div class="bg-silver/10 rounded-xl p-5 space-y-4 border border-silver/30">
					<div class="flex items-center gap-2 text-green-600 font-bold">
						<CheckCircle2 class="h-6 w-6" />
						Proceso finalizado
					</div>
					<div class="grid grid-cols-3 gap-3 text-sm">
						<div class="bg-white p-3 rounded-lg border border-silver/50 shadow-sm text-center">
							<p class="text-[10px] text-grey-olive uppercase tracking-wider font-bold">Creados</p>
							<p class="text-2xl font-black text-green-600">{results.filter(r => r.status === 'ok').length}</p>
						</div>
						<div class="bg-white p-3 rounded-lg border border-silver/50 shadow-sm text-center">
							<p class="text-[10px] text-grey-olive uppercase tracking-wider font-bold">Duplicados</p>
							<p class="text-2xl font-black text-amber-500">{results.filter(r => r.status === 'duplicate').length}</p>
						</div>
						<div class="bg-white p-3 rounded-lg border border-silver/50 shadow-sm text-center">
							<p class="text-[10px] text-grey-olive uppercase tracking-wider font-bold">Errores</p>
							<p class="text-2xl font-black text-red-500">{results.filter(r => r.status === 'error').length}</p>
						</div>
					</div>

					{#if results.some(r => r.status !== 'ok')}
						<div class="bg-white/50 rounded-lg p-3 border border-silver/30 shadow-inner max-h-40 overflow-y-auto custom-scrollbar">
							<p class="text-xs font-bold text-taupe mb-2">Detalle:</p>
							<ul class="text-[10px] space-y-1">
								{#each results.filter(r => r.status !== 'ok') as r}
									<li class="flex items-start gap-1.5">
										{#if r.status === 'duplicate'}
											<AlertCircle class="h-3 w-3 text-amber-500 shrink-0 mt-0.5" />
										{:else}
											<XCircle class="h-3 w-3 text-red-500 shrink-0 mt-0.5" />
										{/if}
										<span class="text-greyOlive break-all">{r.url} — {r.message}</span>
									</li>
								{/each}
							</ul>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<Dialog.Footer class="gap-2">
			{#if !done}
				<Button
					variant="ghost"
					onclick={() => (open = false)}
					disabled={loading}
					class="text-grey-olive hover:text-taupe"
				>
					Cancelar
				</Button>
				<Button
					onclick={startBulkAdd}
					disabled={validUrls.length === 0 || hasErrors || loading}
					class="bg-taupe text-parchment hover:bg-taupe/90 font-bold shadow-md px-6 min-w-[140px]"
				>
					{#if loading}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
						Procesando...
					{:else}
						Añadir {validUrls.length} URL{validUrls.length !== 1 ? 's' : ''}
					{/if}
				</Button>
			{:else}
				<Button
					onclick={() => { open = false; reset(); }}
					class="bg-taupe text-parchment hover:bg-taupe/90 w-full font-bold h-11 shadow-md"
				>
					Finalizar
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 4px;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: #C0C0C0;
		border-radius: 4px;
	}
</style>
