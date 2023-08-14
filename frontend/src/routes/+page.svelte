<script lang="ts">
	import axios from 'axios';
	import { goto } from '$app/navigation';
	import { AppShell } from '@skeletonlabs/skeleton';
	axios.defaults.withCredentials = true;

	let form_username = '';
	let password = '';

	async function handleSubmit() {
		try {
			const response = await axios.post('http://localhost:3000/login', {
				username: form_username,
				password: password
			});
			goto('/dashboard');
		} catch (error) {
			alert('login failed');
		}
	}
</script>

<AppShell title="Login" class="bg-gradient-to-br variant-gradient-secondary-primary">
	<div class="container h-full mx-auto flex justify-center items-center">
		<div class="space-y-5">
			<h1 class="h2 text-center">Login</h1>
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
			<p class="text-sm">
				Don't have an account? <a href="/register" class="underline">Register</a>
			</p>
			<div class="flex justify-center">
				<button type="button" class="btn variant-filled-secondary w-32" on:click={handleSubmit}
					>Sign in</button
				>
			</div>
		</div>
	</div>
</AppShell>
