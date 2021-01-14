<div class="media-viewer" bind:this="{el}" on:click="{close}" data-item-path="{item.path}">
	<div class="media-viewer-img-wrap">
		<img src="{item.thumb}" alt="{item.name || ''}" bind:this="{img}">
	</div>
</div>

<script>
import { onMount } from 'svelte';
import { EVENT, animate, getBox } from '../lib';

let item = { src: '', name: '', type: 'photo' };
let el, img, targetBox;
const fullScreen = { left: 0, top: 0, width: '100%', height: '100%' };

onMount(() => {
	EVENT.on(EVENT.item.view, open);
});


function open (_item, clickedEl) {
	document.documentElement.style.overflow = 'hidden';
	item = _item;
	targetBox = getBox(clickedEl);
	Object.assign(el.style, targetBox, { display: 'block' });

	let full = new Image();
	full.onload = () => {
		// ensure that on slow connection we're showing the right image
		const itemPath = document.querySelector('.media-viewer')?.dataset?.itemPath;
		if (full.src.includes(itemPath)) img.src = full.src;
	};
	animate(el, targetBox, fullScreen).then(() => full.src = _item.path);
}

function close () {
	animate(el, fullScreen, targetBox)
		.then(() => {
			el.style.display = 'none';
			document.documentElement.style.overflow = '';
			img.src = '#';
		});
}
</script>
