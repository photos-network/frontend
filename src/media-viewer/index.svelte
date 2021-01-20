<div class="media-viewer" bind:this="{el}" on:click="{close}" data-item-path="{item.path}">
	<Menu {isOpen} />
	<div class="media-viewer-img-wrap">
		<BtnPrev />
		<img src="{item.thumb}" alt="{item.name || ''}" bind:this="{img}">
		<BtnNext />
	</div>
	<InfoPanel item="{item}"/>
</div>

<script>
import { onMount } from 'svelte';
import Menu from './menu';
import InfoPanel from './info-panel';
import BtnPrev from './btn-prev';
import BtnNext from './btn-next';
import { EVENT, animate, getBoxCenter, items } from '../lib';

let item = { src: '', name: '', type: 'photo' };
let el, img, mediaItemElements, isOpen = false;
const thumbProps = { transform: 'scale(0.2)', opacity: 0 };
const fullScreenProps = { transform: 'scale(1)', opacity: 1 };

onMount(() => {
	mediaItemElements = document.querySelectorAll('.main .media-item');
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
		el.style.display = 'flex';
		el.style.transformOrigin = getBoxCenter(clickedEl);
		document.documentElement.style.overflow = 'hidden';  // hide scrollbars
		await animate(el, thumbProps, fullScreenProps);
	}
	full.src = _item.path;
	isOpen = true;
}

async function close () {
	img.src = item.thumb; // for smoother animation
	const targetElement = Array.from(mediaItemElements).find(i => i.id === item.path);
	el.style.transformOrigin =  getBoxCenter(targetElement);
	await animate(el, fullScreenProps, thumbProps);
	el.style.display = 'none';
	document.documentElement.style.overflow = '';
	img.src = '#';
	isOpen = false;
	requestAnimationFrame(() => targetElement.focus());
}
</script>
