<script lang="ts">
	import Listing from '$lib/School.svelte';
	import type { School } from '$lib/shared';
	import { invoke } from '@tauri-apps/api/core';

	export let on_select = (school: School) => {
		alert(school.name);
	};

	let schools: School[] = [];
	let query = '';
	let include_unsupported = true;

	async function refresh() {
		await invoke('schools_refresh', {});
	}

	async function apply_filters() {
		schools = await invoke('schools_filter', {
			query,
			includeUnsupported: include_unsupported
		});
	}

    async function toggle_unsupported() {
        include_unsupported = !include_unsupported;
        apply_filters();
    }

	refresh();
	apply_filters();
</script>

<div class="school-list">
	<h2>Select school</h2>
	<div class="filters">
        <input type="checkbox" checked={include_unsupported} on:click={toggle_unsupported}>
		<input
			class="large-input"
			type="text"
			placeholder="Type to search"
			bind:value={query}
			on:keyup={apply_filters}
		/>
	</div>
	<div class="list">
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
		display: flex;
		flex-direction: column;
		align-items: center;
		background: white;
		border-radius: 1em;
		border: 1px solid black;
		height: 100%;
		overflow: hidden;
        padding: 1em;

		div.filters {
			display: flex;
			gap: 1em;
            padding: 0 1em;
			width: 100%;

            input[type='checkbox'] {
                height: 3em;
                width: 3em;
                background: red;
                border: 10px solid black;
            }

			input[type='text'] {
				width: 100%;
				padding: 1em 1em;
				border-radius: 0.8em;
				border: 2px solid gray;
				box-shadow: 0 0 5px gray;
			}

			// button {
			// 	background: white;
			// 	padding: 0;
			// 	font-size: 1.5em;
			// 	border-radius: 2em;
			// 	border: 2px solid gray;
			// 	box-shadow: 0 0 5px gray;
			// }
		}

		div.list {
			display: flex;
			flex-direction: column;
			height: 100%;
			overflow: scroll;
			margin: 0;
			padding: 1em;
			gap: 10px;
		}
	}
</style>
