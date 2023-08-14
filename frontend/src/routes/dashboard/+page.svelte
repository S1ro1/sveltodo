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

	let search = '';

	$: filteredTasks = $tasks.filter((t) => t.title.toLowerCase().includes(search.toLowerCase()));

	onMount(async () => {
		const response = await axios.get<ResponseTask[]>('http://localhost:3000/get_user_tasks');
		tasks.set(response.data);
	});

	async function deleteTask(e: MouseEvent, id: number) {
		e.stopPropagation();
		try {
			const response = await axios.delete(`http://localhost:3000/delete_task/${id}`);
			tasks.update((t) => t.filter((t) => t.id !== id));
		} catch (e) {
			alert('Error deleting task');
		}
	}

	function modalComponentForm(
		taskId: number,
		taskTitle: string,
		taskDescription: string,
		taskDifficulty: number,
		taskUpdate: boolean
	): void {
		const c: ModalComponent = { ref: ModalExampleForm };
		const modal: ModalSettings = {
			type: 'component',
			component: c,
			title: 'Create task',
			body: 'Fill task information and press submit',
			meta: {
				task_id: taskId,
				task_title: taskTitle,
				task_description: taskDescription,
				task_difficulty: taskDifficulty,
				update: taskUpdate
			}
		};
		modalStore.trigger(modal);
	}

	const modalComponentRegistry: Record<string, ModalComponent> = {
		modalComponentOne: {
			ref: ModalExampleForm
		}
	};

	async function toggleTaskStatus(task: ResponseTask) {
		try {
			const response = await axios.put(`http://localhost:3000/update_task_status/${task.id}`, {
				finished: !task.finished
			});
			tasks.update((t) => {
				const index = t.findIndex((t) => t.id === task.id);
				t[index].finished = !t[index].finished;
				return t;
			});
		} catch (e) {
			alert('Error updating task status');
		}
	}
</script>

<Modal components={modalComponentRegistry} />
<AppShell title="Dashboard" class="bg-gradient-to-br variant-gradient-secondary-primary">
	<svelte:fragment slot="header">
		<div class="flex justify-end p-8">
			<Avatar initials={$username} />
		</div>
	</svelte:fragment>
	<slot>
		<div class="container h-full mx-auto flex justify-center items-center">
			<div class="flex-col justify-center w-2/3">
				<div>
					<input
						type="text"
						class="input variant-ghost-primary placeholder-inherit"
						placeholder="Search"
						bind:value={search}
					/>
				</div>
				{#each filteredTasks as task}
					<button
						class="appearance-none border-none bg-none p-0 m-0 block w-full text-left"
						on:click={() => {
							modalComponentForm(task.id, task.title, task.description, task.difficulty, true);
						}}
					>
						<div class="card p-4 m-4 variant-soft-surface card-hover">
							<header class="class-header font-extrabold border-1 flex justify-between">
								<p>{task.title.toUpperCase()}</p>
								<button
									type="button"
									class="btw-icon text-red"
									on:click={async (e) => {
										await deleteTask(e, task.id);
									}}>x</button
								>
							</header>
							<section class="p-4">{task.description}</section>
							<section class="card-footer">
								<button
									type="button"
									class="btn btn-sm variant-filled-secondary"
									on:click={(e) => {
										e.stopPropagation();
										toggleTaskStatus(task);
									}}
								>
									{task.finished ? 'Completed' : 'Pending'}
								</button>
								<Ratings value={task.difficulty} max={5} justify="justify-end">
									<svelte:fragment slot="empty">{@html icons.circleEmpty}</svelte:fragment>
									<svelte:fragment slot="half">{@html icons.circleHalf}</svelte:fragment>
									<svelte:fragment slot="full">{@html icons.circleFull}</svelte:fragment>
								</Ratings>
							</section>
						</div>
					</button>
				{/each}
			</div>
		</div>
	</slot>
	<svelte:fragment slot="pageFooter">
		<div class="flex justify-end p-8">
			<button
				type="button"
				class="btn btn-xl variant-filled-secondary"
				on:click={() => {
					modalComponentForm(-1, '', '', 0, false);
				}}>+</button
			>
		</div>
	</svelte:fragment>
</AppShell>