<script lang="ts">
	import type { School } from './shared';

	export let school: School;
	export let on_click = () => {};
</script>

<button class="school" on:click={on_click}>
	<p>{school.name}</p>
</button>
