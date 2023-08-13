<script lang="ts">
	import axios from 'axios';
	import { goto } from '$app/navigation';
	import { AppShell } from '@skeletonlabs/skeleton';
	axios.defaults.withCredentials = true;

	let form_username = '';
	let password = '';
	let repeated_password = '';
	let failed_register = false;

	async function handleSubmit() {
		try {
			const response = await axios.post('http://localhost:3000/register', {
				username: form_username,
				password: password,
				repeated_password: repeated_password
			});

			goto('/');
		} catch (error) {
			alert('Register failed');
		}
	}
</script>

<AppShell title="Login" class="bg-gradient-to-br variant-gradient-secondary-primary">
	<div class="container h-full mx-auto flex justify-center items-center">
		<div class="space-y-5">
			<h1 class="h2 text-center">Register</h1>
			<input
				class="input"
				title="username"
				type="text"
				placeholder="username"
				bind:value={form_username}
			/>
			<input
				class="input"
				title="password"
				type="password"
				placeholder="password"
				bind:value={password}
			/>
			<input
				class="input"
				title="password"
				type="password"
				placeholder="repeate password"
				bind:value={repeated_password}
			/>
			{#if failed_register}
				<p class="text-sm text-red-500">Register failed</p>
			{/if}
			<div class="flex justify-center">
				<button type="button" class="btn variant-filled-secondary w-32" on:click={handleSubmit}
					>Sign up</button
				>
			</div>
		</div>
	</div>
</AppShell>
