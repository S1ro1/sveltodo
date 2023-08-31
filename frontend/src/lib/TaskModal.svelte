<script lang="ts">
	import { RangeSlider, modalStore } from '@skeletonlabs/skeleton';
	import axios, { type AxiosResponse } from 'axios';
	import { tasks } from '../store';
	import type { ResponseTask } from '../routes/dashboard/tasks';
	export let parent: any;

	async function onFormSubmit() {
		let response: AxiosResponse<ResponseTask>;

		if (!$modalStore[0].meta.update) {
			try {
				response = await axios.post<ResponseTask>('http://localhost:3000/create_task', formData);
				tasks.update((t) => [...t, response.data]);
			} catch (e) {
				alert('Error creating task');
			}
		} else {
			if (!$modalStore[0].meta.task_id) {
				alert("Error updating task, task id doesn't exist");
				return;
			}

			let id = $modalStore[0].meta.task_id;
			try {
				response = await axios.put<ResponseTask>(
					`http://localhost:3000/update_task/${id}`,
					formData
				);
				tasks.update((t) => {
					let index = t.findIndex((t) => t.id === id);
					t[index] = response.data;
					return t;
				});
			} catch (e) {
				alert('Error updating task');
			}
		}

		modalStore.close();
	}

	const formData = {
		title: $modalStore[0].meta.task_title ?? '',
		description: $modalStore[0].meta.task_description ?? '',
		difficulty: $modalStore[0].meta.task_difficulty ?? 0
	};

	// Base Classes
	const cBase = 'card p-4 w-modal shadow-xl space-y-4';
	const cHeader = 'text-2xl font-bold';
	const cForm = 'border border-surface-500 p-4 space-y-4 rounded-container-token';
</script>

<!-- @component This example creates a simple form modal. -->

{#if $modalStore[0]}
	<div class="modal-example-form {cBase}">
		<header class={cHeader}>{$modalStore[0].title ?? '(title missing)'}</header>
		<article>{$modalStore[0].body ?? '(body missing)'}</article>
		<form class="modal-form {cForm}">
			<label class="label">
				<span>Title</span>
				<input
					class="input"
					type="text"
					bind:value={formData.title}
					placeholder="Enter task title"
				/>
			</label>
			<label class="label">
				<span>Description</span>
				<textarea
					class="textarea"
					bind:value={formData.description}
					placeholder="Enter task description..."
				/>
			</label>
			<label class="label">
				<span>Difficulty</span>
				<input type="hidden" bind:value={formData.difficulty} />
				<RangeSlider ticked name="range-slider" bind:value={formData.difficulty} min={0} max={5} />
			</label>
		</form>
		<!-- prettier-ignore -->
		<footer class="modal-footer {parent.regionFooter}">
        <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{parent.buttonTextCancel}</button>
        <button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>{$modalStore[0].meta.update ? 'Update task' : 'Create task'}</button>
    </footer>
	</div>
{/if}
