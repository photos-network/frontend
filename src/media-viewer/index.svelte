<div class="media-viewer" bind:this="{el}" data-item-path="{item.path}">
	<Menu {isOpen} />
	<div class="media-viewer-img-wrap">
		<img src="{item.thumb}" alt="{item.name || ''}" bind:this="{img}">
	</div>
</div>

<script>
import { onMount } from 'svelte';
import Menu from './menu';
import { EVENT, animate, getBox, items } from '../lib';

let item = { src: '', name: '', type: 'photo' };
let el, img, mediaItemElements, isOpen = false;
const fullScreen = { left: 0, top: 0, width: '100%', height: '100%', backgroundColor: '#111' };


onMount(() => {
	mediaItemElements = document.querySelectorAll('.main .media-item');
	console.log(123);
	EVENT.on(EVENT.item.view, open);
	EVENT.on(EVENT.item.close, close);
	EVENT.on(EVENT.item.prev, prev);
	EVENT.on(EVENT.item.next, next);
});


function prev () {
	const idx = Array.from(mediaItemElements).findIndex(i => i.id === item.path);
	const nodeId = (idx > 0 ? mediaItemElements[idx - 1] : mediaItemElements[idx]).id;
	const _item = $items.find(i => i.path === nodeId);
	EVENT.fire(EVENT.item.view, _item);
}

function next () {
	const idx = Array.from(mediaItemElements).findIndex(i => i.id === item.path);
	const nodeId = (idx < mediaItemElements.length ? mediaItemElements[idx + 1] : mediaItemElements[idx]).id;
	const _item = $items.find(i => i.path === nodeId);
	EVENT.fire(EVENT.item.view, _item);
}


function imgOnLoad (e) {
	// ensure that on slow connection we're showing the right image
	const itemPath = document.querySelector('.media-viewer')?.dataset?.itemPath;
	if (e.target.src.includes(itemPath)) img.src = e.target.src;
}

async function open (_item, clickedEl) {
	item = _item;

	let full = new Image();
	full.onload = imgOnLoad;

	if (!isOpen) {
		mediaItemElements = document.querySelectorAll('.main .media-item');
		const targetBox = Object.assign(getBox(clickedEl), { backgroundColor: 'transparent' });
		Object.assign(el.style, targetBox, { display: 'block' });
		document.documentElement.style.overflow = 'hidden';
		await animate(el, targetBox, fullScreen);
	}
	full.src = _item.path;
	isOpen = true;
}

async function close () {
	img.src = item.thumb; // for smoother animation
	const targetElement = Array.from(mediaItemElements).find(i => i.id === item.path);
	const targetBox = Object.assign(getBox(targetElement), { backgroundColor: 'transparent' });
	await animate(el, fullScreen, targetBox);
	el.style.display = 'none';
	document.documentElement.style.overflow = '';
	img.src = '#';
	isOpen = false;
	requestAnimationFrame(() => targetElement.focus());
}
</script>
