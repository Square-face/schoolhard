<script lang="ts">
	import type { School } from '$lib/shared';
	import List from './list.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let school: School | null = null;
	let username = '';
	let password = '';
	let selecting = true;

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
		<h1>Login</h1>

		<button
			on:click={() => {
				selecting = true;
			}}
		>
			{#if school == null}
				Select school
			{:else}
				{school.name}
			{/if}
		</button>

		<input id="username" type="text" placeholder="Username" bind:value={username} />

		<input id="password" type="password" placeholder="Password" bind:value={password} />

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
		position: relative;
		height: 100vh;
		width: 100vw;

		display: flex;
		justify-content: center;
		align-items: center;

		margin: 0;
		padding: 0;
		overflow: hidden;

		.login {
			display: flex;
			gap: 0.7em;
			flex-direction: column;
			align-items: center;
			padding: 1em 2em;

			border-radius: 0.7em;
			box-shadow: 0 0 5px rgba(0, 0, 0, 0.6);

			input[type='text'],
			input[type='password'] {
				padding: 0.4em 0.8em;
				border-radius: 2.6em;
				border: 1px solid rgba(0, 0, 0, 0.6);
			}

			button {
				padding: 0.4em 0.8em;
				border-radius: 2.6em;
				border: 1px solid rgba(0, 0, 0, 0.6);
				background: transparent;

				&:hover {
					background: rgba(0, 0, 0, 0.2);
				}
			}
		}

		.schoollist {
			position: absolute;
			top: 0;
			left: 0;

			// calculate width and height manually to avoid scroll bars
			// unfortunately max-height doesn't work for some reason
			height: calc(100vh - 4em);
			width: calc(100vw - 4em);

			margin: 2em;
			padding: 0;
			overflow: hidden;

			border-radius: 1em;
			box-shadow: 1px 1px 5px rgba(0, 0, 0, 0.6);
		}
	}
</style>
