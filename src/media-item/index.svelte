<a href="#{src.path}" class="media-item {src.type || 'photo'}"
		class:loading="{loading}"
		bind:this="{el}"
		on:click|preventDefault="{viewItem}"
		id="{src.path}">
	<img bind:this="{img}" src="#" alt="{src.name || ''}">
</a>


<script>
import { onMount } from 'svelte';
import { EVENT, inView } from '../lib';
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


function viewItem () {
	EVENT.fire(EVENT.item.view, src, el);
}

</script>
