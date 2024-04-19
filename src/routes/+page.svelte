<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	let logged_in = false;

	async function check() {
		logged_in = await invoke('is_logged_in', {});
	}

	check();
</script>

<h1>Schoolhard</h1>
{#if logged_in}
	<p>
		Lorem ipsum dolor, sit amet consectetur adipisicing elit. Laboriosam quod fugiat aliquam ullam
		officiis, repellat dolorum commodi quis inventore possimus explicabo ipsa omnis totam tempora
		magni autem sequi? Dolores, perferendis.
	</p>
{:else}
	<a href="/login">Login</a>
{/if}
