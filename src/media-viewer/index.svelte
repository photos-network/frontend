<div class="media-viewer" bind:this="{el}" on:click="{close}" data-item-path="{item.path}">
	<a href="#prev" class="icon-btn btn-prev" title="Previous"
		on:click|preventDefault|stopPropagation="{prev}">
		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-left" width="84" height="84" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<line x1="5" y1="12" x2="19" y2="12" />
			<line x1="5" y1="12" x2="11" y2="18" />
			<line x1="5" y1="12" x2="11" y2="6" />
		</svg>
	</a>

	<div class="media-viewer-img-wrap">
		<img src="{item.thumb}" alt="{item.name || ''}" bind:this="{img}">
	</div>

	<a href="#next" class="icon-btn btn-next" title="Next"
		on:click|preventDefault|stopPropagation="{next}">
		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-right" width="84" height="84" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<line x1="5" y1="12" x2="19" y2="12" />
			<line x1="13" y1="18" x2="19" y2="12" />
			<line x1="13" y1="6" x2="19" y2="12" />
		</svg>
	</a>
</div>

<script>
import { onMount } from 'svelte';
import { EVENT, animate, getBox, items } from '../lib';

let item = { src: '', name: '', type: 'photo' };
let el, img, mediaItemElements, isOpen = false;
const fullScreen = { left: 0, top: 0, width: '100%', height: '100%', backgroundColor: '#111' };
const keyMap = {
	ArrowLeft: prev,
	ArrowRight: next,
	Escape: close,
};


onMount(() => {
	mediaItemElements = document.querySelectorAll('.main .media-item');
	document.addEventListener('keydown', onKeydown, true);
	EVENT.on(EVENT.item.view, open);
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

function onKeydown (e) {
	if (!isOpen) return;
	if (typeof keyMap[e.key] === 'function') keyMap[e.key](e);
}
</script>
