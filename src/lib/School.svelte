<script lang="ts">
	import type { School } from './shared';

	export let school: School;
	export let on_click = () => {};
</script>

<button class="school" on:click={on_click}>
	<p>{school.name}</p>
    {#if !school.supported}
        <span class="unsupported">*</span>
    {/if}
</button>

<style lang="scss">
	button.school {
        display: flex;
        background: white;
        border: none;
        margin: 0;
        padding: 0 1em;
        width: 100%;
        box-shadow: 1px 1px 4px rgba(0, 0, 0, 0.6);
        justify-content: space-between;
        align-items: center;

        &:hover {
            background: lightgray;
        }

        .unsupported {
            color: red;
        }
	}
</style>
