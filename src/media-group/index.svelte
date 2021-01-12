<h2 class="media-group-title">{group.title}</h2>
<div class="media-group" bind:this="{elem}">
	{#each group.items as item}
		<MediaItem src="{item}" />
	{/each}
</div>


<script>
import MediaItem from '../media-item';

export let group = { title: '', items: [] };
let elem;


import { onMount } from 'svelte';
import * as basicScroll from 'basicscroll';

onMount(() => {
	const instance = basicScroll.create({
		elem,
		from: 'top-bottom',
		to: 'bottom-top',
		direct: true,
		props: { '--scroll-offset': { from: '0px', to: '60px' } }
	});
	instance.start();
});

</script>
