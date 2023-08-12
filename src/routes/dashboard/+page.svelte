<script lang="ts">
	import axios from 'axios';
	import { AppShell, Ratings } from '@skeletonlabs/skeleton';
	import { icons } from './icons';
	import { onMount } from 'svelte';
	import type { RequestTask, ResponseTask } from './tasks';

	let tasks: ResponseTask[] = [];

	onMount(async () => {
		const response = await axios.get<ResponseTask[]>('http://localhost:3000/get_user_tasks');
		tasks = response.data;
	});

	async function addTask() {
		const task: RequestTask = {
			title: 'New Task235231',
			description: 'New Task Description',
			difficulty: 1
		};
		console.log(task);

		const response = await axios.post<ResponseTask>('http://localhost:3000/create_task', task);

		if (response.status !== 200) {
			console.log('Error creating task');
			return;
		}
		const new_task = response.data;

		tasks = [...tasks, new_task];
	}
</script>

<AppShell title="Dashboard" class="bg-gradient-to-br variant-gradient-secondary-primary">
	<slot>
		<div class="container h-full mx-auto flex justify-center items-center">
			<div class="flex-col justify-center">
				{#each tasks as task}
					<div class="card p-4 m-4 variant-soft-surface">
						<header class="class-header font-extrabold border-1 flex justify-between">
							<p>{task.title.toUpperCase()}</p>
							<button type="button" class="btw-icon text-red">x</button>
						</header>
						<section class="p-4">{task.description}</section>
						<section class="card-footer">
							<Ratings value={task.difficulty} max={5} justify="justify-end">
								<svelte:fragment slot="empty">{@html icons.circleEmpty}</svelte:fragment>
								<svelte:fragment slot="half">{@html icons.circleHalf}</svelte:fragment>
								<svelte:fragment slot="full">{@html icons.circleFull}</svelte:fragment>
							</Ratings>
						</section>
					</div>
				{/each}
			</div>
		</div>
	</slot>
	<svelte:fragment slot="pageFooter">
		<div class="flex justify-end p-20">
			<button type="button" class="btn btn-xl variant-filled-secondary" on:click={addTask}>+</button
			>
		</div>
	</svelte:fragment>
</AppShell>
