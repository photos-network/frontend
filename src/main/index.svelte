<svelte:window on:popstate={onpopstate}/>

<main class="main">
	{#if $initialised}
		{#if $items}
			{#each $items as item}
				<img class="media-item" src="{item.path}" alt="{item.name}">
			{/each}
		{:else}
			Add some photos
		{/if}
	{:else}
		Loading...
	{/if}
</main>


<script>
import { initialised, items } from '../lib';

function onpopstate (e) {
	if (e.state) console.log(e.state.id);
}

</script>
