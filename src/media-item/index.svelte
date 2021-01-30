<a href="#{src.path}" class="media-item {src.type || 'photo'}"
		class:loading="{loading}"
		bind:this="{elem}"
		on:click|preventDefault="{viewItem}"
		id="{src.path}">
	<img bind:this="{img}" src="#" alt="{src.name || ''}">
	<Spinner />
</a>


<script>
import { onMount, onDestroy } from 'svelte';
import { EVENT, inView } from '../lib';
import * as basicScroll from 'basicscroll';
import Spinner from '../spinner';
export let src = { src: '', name: 'Loading...', type: 'photo' };

let loading = false;
let elem, img, instance;

function viewItem () {
	EVENT.fire(EVENT.item.view, src, elem);
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


	inView(elem, () => {
		loading = true;
		img.src = src.thumb || src.path || '#';
		img.onload = () => requestAnimationFrame(() => loading = false);
	});
});

onDestroy(() => {
	instance?.destroy();
});

</script>
