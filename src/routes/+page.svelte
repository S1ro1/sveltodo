<!-- YOU CAN DELETE EVERYTHING IN THIS PAGE -->

<script lang="ts">
	import axios from 'axios';
	import { goto } from '$app/navigation';
	import { username } from '../store';
	axios.defaults.withCredentials = true;

	let form_username = '';
	let password = '';

	async function handleSubmit() {
		const response = await axios.post('http://localhost:3000/login', {
			username: form_username,
			password: password
		});

		if (response.status == 200) {
			username.set(form_username);	

			goto('/dashboard');
		} else {
			alert('login failed');
		}
	}
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-5">
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
		<div class="flex justify-center">
			<button type="button" class="btn variant-filled w-32" on:click={handleSubmit}>login</button>
		</div>
	</div>
</div>
