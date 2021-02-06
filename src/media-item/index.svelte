<a href="#timeline/preview/{src.id}" class="media-item {src.type || 'photo'}"
	class:loading="{loading}"
	bind:this="{elem}"
	id="{src.id}">
		<img bind:this="{img}" src="data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7" alt="{src.name || ''}">
		<Spinner />
</a>


<script>
import { onMount, onDestroy } from 'svelte';
import { inView } from '../lib';
import * as basicScroll from 'basicscroll';
import Spinner from '../spinner';
export let src = { src: '', name: 'Loading...', type: 'photo' };

let loading = false;
let elem, img, instance;


onMount(() => {
	// create parallax/depth effect on photos (scroll them slower than the page)
	instance = basicScroll.create({
		elem,
		from: 'top-bottom',
		to: 'bottom-top',
		direct: true,
		props: { '--scroll-offset': { from: '0px', to: '60px' } },
	});
	instance.start();

	inView(elem, () => {
		loading = true;
		img.src = src.thumb || src.path || '#';
		img.onload = () => requestAnimationFrame(() => loading = false);
	});

	requestAnimationFrame(() => {
		instance.calculate();
		instance.update();
	});
});

onDestroy(() => {
	instance?.destroy();
});

</script>
