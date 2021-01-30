{#if group.title}
	<h2 class="media-group-title">{group.title}</h2>
{/if}
<div class="media-group" bind:this="{elem}">
	{#each group.items as item}
		<MediaItem src="{item}" />
	{/each}
</div>


<script>
import MediaItem from '../media-item';

export let group = { title: '', items: [] };
let elem, instance;


import { onDestroy, onMount } from 'svelte';
import * as basicScroll from 'basicscroll';

$:{
	if (group.items.length) requestAnimationFrame(updateBasicScroll);
}

function updateBasicScroll () {
	instance?.calculate();
	instance?.update();
}

onMount(() => {
	// create parallax/depth effect on photos (scroll them slower than the page)
	instance = basicScroll.create({
		elem,
		from: 'top-bottom',
		to: 'bottom-top',
		direct: true,
		props: { '--scroll-offset': { from: '0px', to: '60px' } }
	});
	instance.start();
});

onDestroy(() => {
	instance?.destroy();
});

</script>
