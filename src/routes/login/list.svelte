<script lang="ts">
	import Listing from '$lib/School.svelte';
	import type { School } from '$lib/shared';
	import { invoke } from '@tauri-apps/api/core';

	export let on_select = (school: School) => {
		alert(school.name);
	};

	let schools: School[] = [];
	let query = '';
	let include_unsupported = false;

	async function refresh() {
		await invoke('schools_refresh', {});
	}

	async function apply_filters() {
		schools = await invoke('schools_filter', {
			query,
			includeUnsupported: include_unsupported
		});
	}

	refresh();
	apply_filters();
</script>

<div class="school-list">
	<h2>Select school</h2>
    <button on:click={refresh}>Refresh</button>
	<input type="text" bind:value={query} on:keyup={apply_filters} />
	<ul>
		{#each schools as school}
			<Listing
				{school}
				on_click={() => {
					on_select(school);
				}}
			/>
		{/each}
	</ul>
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

		ul {
			height: 100%;
			overflow: scroll;
		}
	}
</style>
