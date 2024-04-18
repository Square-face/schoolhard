<script lang="ts">
	import Listing from '$lib/School.svelte';
	import type { School } from '$lib/shared';
	import { invoke } from '@tauri-apps/api/core';

	export let on_select = (school: School) => {
		alert(school.name);
	};

	let schools: School[] = [];
	let query = '';
	let loading = false;

	async function refresh() {
		loading = true;
		await invoke('schools_refresh', {});
		loading = false;
	}

	async function apply_filters() {
		schools = await invoke('schools_filter', {
			query,
			includeUnsupported: true
		});
	}

	refresh();
	apply_filters();
</script>

<div class="school-list">
	<input
		class="large-input"
		type="text"
		placeholder="Type to search"
		bind:value={query}
		on:keyup={apply_filters}
	/>
	<div class="list">
		{#if loading}
			<p>Loading...</p>
		{/if}
		{#each schools as school}
			<Listing
				{school}
				on_click={() => {
					on_select(school);
				}}
			/>
		{/each}
	</div>
</div>

<style lang="scss">
	.school-list {
		display: grid;

		column-gap: 1em;

		grid-template-rows: auto auto 1fr;
		grid-template-columns: auto 1fr;
		grid-template-areas: 't t' 'q q' 'l l';

		height: 100%;
		padding: 0;

		background: white;
		overflow: hidden;

		input[type='text'] {
			grid-area: q;

			padding: 1em;
			margin: 1em 2em 0.1em 2em;

			border: none;
			border-bottom: 2px solid gray;

            font-size: 1.1em;

			&:focus {
				outline: none;
				border-bottom: 2px solid rgb(110, 120, 255);
			}
		}

		div.list {
			grid-area: l;

			display: flex;
			flex-direction: column;
			gap: 10px;

			padding: 0 1em;
			margin: 1em;
			overflow: scroll;
		}
	}
</style>
