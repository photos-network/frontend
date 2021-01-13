<div class="media-item {src.type || 'photo'}" class:loading="{loading}" bind:this="{el}" >
	<img bind:this="{img}" src="#" alt="{src.name || ''}">
</div>


<script>
import { onMount } from 'svelte';
import { inView } from '../lib';
export let src = { src: '', name: 'Loading...', type: 'photo' };

let loading = false;
let el, img;

onMount(() => {
	inView(el, () => {
		loading = true;
		img.src = src.thumb || src.path || '#';
		img.onload = () => requestAnimationFrame(() => loading = false);
	});
});

</script>
