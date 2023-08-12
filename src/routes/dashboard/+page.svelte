<script lang="ts">
	import axios from 'axios';
	import {
		AppShell,
		Ratings,
		type ModalComponent,
		type ModalSettings,
		modalStore,
		Modal,
		Avatar
	} from '@skeletonlabs/skeleton';
	import { icons } from './icons';
	import { onMount } from 'svelte';
	import type { RequestTask, ResponseTask } from './tasks';
	import ModalExampleForm from '$lib/ModalExampleForm.svelte';
	import { tasks, username } from '../../store';

	onMount(async () => {
		const response = await axios.get<ResponseTask[]>('http://localhost:3000/get_user_tasks');
		tasks.set(response.data);
	});

	async function deleteTask(id: number) {
		const response = await axios.delete(`http://localhost:3000/delete_task/${id}`);

		if (response.status !== 200) {
			console.log('Error deleting task');
			return;
		}

		tasks.update((t) => t.filter((t) => t.id !== id));
	}

	function modalComponentForm(): void {
		const c: ModalComponent = { ref: ModalExampleForm };
		const modal: ModalSettings = {
			type: 'component',
			component: c,
			title: 'Create task',
			body: 'Fill task information and press submit',
			response: (r: any) => console.log('response:', r)
		};
		modalStore.trigger(modal);
	}

	const modalComponentRegistry: Record<string, ModalComponent> = {
		modalComponentOne: {
			ref: ModalExampleForm
		}
	};
</script>

<Modal components={modalComponentRegistry} />
<AppShell title="Dashboard" class="bg-gradient-to-br variant-gradient-secondary-primary">
	<svelte:fragment slot="header">
		<div class="flex justify-end p-4">
			<Avatar initials={$username} />
		</div>
	</svelte:fragment>
	<slot>
		<div class="container h-full mx-auto flex justify-center items-center">
			<div class="flex-col justify-center w-2/3">
				{#each $tasks as task}
					<div class="card p-4 m-4 variant-soft-surface">
						<header class="class-header font-extrabold border-1 flex justify-between">
							<p>{task.title.toUpperCase()}</p>
							<button
								type="button"
								class="btw-icon text-red"
								on:click={async () => {
									await deleteTask(task.id);
								}}>x</button
							>
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
			<button
				type="button"
				class="btn btn-xl variant-filled-secondary"
				on:click={modalComponentForm}>+</button
			>
		</div>
	</svelte:fragment>
</AppShell>
