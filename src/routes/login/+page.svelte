<script lang="ts">
	import type { School } from '$lib/shared';
	import List from './list.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let school: School | null = null;
	let username = '';
	let password = '';
	let selecting = false;

	async function login() {
		await invoke('login', {
			password,
			username,
			school: school?.url_name
		})
			.then((r) => {
				alert(`hello ${r}`);
			})
			.catch((r) => {
				alert(r);
			});
	}
</script>

<div class="page">
	<div class="login">
		{#if school == null}
			<label for="school">School</label>
		{:else}
			<label for="school">School: {school.name}</label>
		{/if}
		<button
			on:click={() => {
				selecting = true;
			}}>Select</button
		>

		<label for="username">Username</label>
		<input id="username" type="text" bind:value={username} />

		<label for="password">Password</label>
		<input id="password" type="password" bind:value={password} />

		<button on:click={login}>Login</button>
	</div>

	{#if selecting}
		<div class="schoollist">
			<List
				on_select={(new_school) => {
					selecting = false;
					school = new_school;
				}}
			/>
		</div>
	{/if}
</div>

<style lang="scss">
	.page {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 100vh;
		position: relative;

		.login {
			display: flex;
			flex-direction: column;
		}
		.schoollist {
			position: absolute;
			top: 0;
			left: 0;
			box-sizing: border-box;
			height: calc(100vh - 3em);
            width: calc(100vw - 3em);
			margin: 1em;
		}
	}
</style>
