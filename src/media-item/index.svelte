<div class="media-item {src.type || 'photo'}" class:loading="{loading}" bind:this="{el}" >
	<img bind:this="{img}" src="#" alt="{src.name || ''}">
</div>


<script>
import { onMount } from 'svelte';
export let src = { src: '', name: 'Loading...', type: 'photo' };

let loading = false;
let el, img;

onMount(() => {
	const options = { rootMargin: '300px', threshold: 1.0 };
	const observer = new IntersectionObserver(entries => {
		entries.forEach(entry => {
			if (entry.intersectionRatio > 0.9) {
				loading = true;
				observer.unobserve(entry.target);
				img.src = src.thumb || src.path || '#';
				img.onload = () => setTimeout(() => loading = false, 500);
			}
		});
	}, options);
	observer.observe(el);
});



</script>
