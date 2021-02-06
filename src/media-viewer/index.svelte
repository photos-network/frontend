<div class="media-viewer" bind:this="{el}" data-item-path="{item.path}">
	<Menu {isOpen} />
	<div class="media-viewer-img-wrap">
		{#if currentItemIndex > 0}
			<BtnPrev />
		{/if}
		{#if item.type === 'video'}
			<div class="video-wrapper" bind:this="{vidWrapperEl}"></div>
		{:else}
			<img src="{item.thumb}" alt="{item.name || ''}" bind:this="{imgEl}">
		{/if}
		{#if currentItemIndex < mediaItemElements?.length - 1}
			<BtnNext />
		{/if}
	</div>
	<InfoPanel item="{item}"/>
</div>

<script>
import { onMount, tick } from 'svelte';
import Menu from './menu';
import InfoPanel from './info-panel';
import BtnPrev from './btn-prev';
import BtnNext from './btn-next';
import { EVENT, animate, getBoxCenter, items, sleep } from '../lib';

let item = { src: '', name: '', type: 'photo' };
let currentItemIndex = 0;
let el, imgEl, vidEl, vidWrapperEl;
let mediaItemElements, isOpen = false;
const thumbProps = { transform: 'scale(0.1)', opacity: 0 };
const fullScreenProps = { transform: 'scale(1)', opacity: 1 };


onMount(() => {
	EVENT.on(EVENT.nav.afterChange, onNavChange);
});


async function itemsRendered () {
	return new Promise(resolve => {
		let unsub;
		unsub = items.subscribe(async () => {
			if (typeof unsub !== 'function') return;
			unsub();
			await sleep(500); // ugly but required
			resolve();
		});
	});
}


async function onNavChange (nav) {
	if (!$items.length) await itemsRendered();

	mediaItemElements = document.querySelectorAll('.main .media-item');
	if (isOpen && nav.action !== 'preview') return close();
	if (nav.action === 'preview' && nav.id) {
		const _item = $items.find(i => i.id === nav.id);
		return open(_item);
	}
}


function getCurrentItemIndex () {
	if (!mediaItemElements) mediaItemElements = document.querySelectorAll('.main .media-item');
	if (!mediaItemElements?.length) return 0;
	return Array.from(mediaItemElements).findIndex(i => i.id === item.id);
}


function imgOnLoad (e) {
	// ensure that on slow connection we're showing the right image
	const itemPath = document.querySelector('.media-viewer')?.dataset?.itemPath;
	if (imgEl && e.target.src.includes(itemPath)) imgEl.src = e.target.src;
}

function resetVideo () {
	if (!vidEl) return;
	vidEl.pause();
	vidEl.remove();
}

/**
 * This manual process is required as in a html template
 * svelte would only replace what has changed, so src & poster attribute values
 * however, to fully "reset" a video tag (to show the poster again) it must be
 * removed and re-added to the DOM
 */
function createVideo () {
	vidEl = document.createElement('VIDEO');
	vidEl.poster = item.thumb;
	vidEl.controls = 'controls';
	vidWrapperEl.appendChild(vidEl);
	tick().then(() => {
		const src = document.createElement('SOURCE');
		src.src = item.path;
		vidEl.appendChild(src);
	});
}



async function open (_item) {
	resetVideo();
	item = _item;
	let full;

	const targetElement = document.querySelector(`.main .media-item[id=${item.id}`);

	if (item.type === 'video') await tick().then(createVideo);
	else {
		full = new Image();
		full.onload = imgOnLoad;
	}

	if (!isOpen) {
		el.style.display = 'flex';
		el.style.transformOrigin = getBoxCenter(targetElement);
		document.documentElement.style.overflow = 'hidden';  // hide scrollbars
		await animate(el, thumbProps, fullScreenProps);
	}
	if (full) full.src = _item.path;
	isOpen = true;
	currentItemIndex = getCurrentItemIndex();
	if (item.type === 'video') tick().then(() => vidEl.focus());
}

async function close () {
	if (imgEl) imgEl.src = item.thumb; // for smoother animation
	resetVideo();
	const targetElement = document.querySelector(`.main .media-item[id=${item.id}`);
	el.style.transformOrigin =  getBoxCenter(targetElement);
	await animate(el, fullScreenProps, thumbProps);
	el.style.display = 'none';
	document.documentElement.style.overflow = '';

	if (imgEl) imgEl.src = '#';
	isOpen = false;
	tick().then(() => targetElement.focus());
}
</script>
