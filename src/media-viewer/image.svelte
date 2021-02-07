<img src="{item.thumb}" alt="{item.name || ''}" bind:this="{el}">

<script>
import { afterUpdate } from 'svelte';
import { sleep } from '../lib';

export let item = { src: '', name: '', type: 'photo' };
let el, fullSize;


afterUpdate(async () => {
	const alreadyFullSize = fullSize && fullSize.src.split('/').pop() === item.path;
	if (!item.path || alreadyFullSize) return;

	fullSize = new Image();
	fullSize.onload = imgOnLoad;
	await sleep(300);
	fullSize.src = item.path;	// this retriggers afterUpdate
});


function imgOnLoad (e) {
	// ensure that on slow connection we're showing the right image
	const itemPath = document.querySelector('.media-viewer')?.dataset?.itemPath;
	if (el && e.target.src.includes(itemPath)) el.src = e.target.src;
}

</script>
