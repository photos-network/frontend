<div class="media-viewer" bind:this="{el}" on:click="{close}" data-item-path="{item.path}">
	<Menu {isOpen} />
	<div class="media-viewer-img-wrap">
		<BtnPrev />
		{#if item.type === 'video'}
			<div class="video-wrapper" bind:this="{vidWrapperEl}"></div>
		{:else}
			<img src="{item.thumb}" alt="{item.name || ''}" bind:this="{imgEl}">
		{/if}
		<BtnNext />
	</div>
	<InfoPanel item="{item}"/>
</div>

<script>
import { onMount, tick } from 'svelte';
import Menu from './menu';
import InfoPanel from './info-panel';
import BtnPrev from './btn-prev';
import BtnNext from './btn-next';
import { EVENT, animate, getBoxCenter, items } from '../lib';

let item = { src: '', name: '', type: 'photo' };
let el, imgEl, vidEl, vidWrapperEl;
let mediaItemElements, isOpen = false;
const thumbProps = { transform: 'scale(0.1)', opacity: 0 };
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
	if (imgEl && e.target.src.includes(itemPath)) imgEl.src = e.target.src;
}

function resetVideo () {
	if (!vidEl) return;
	vidEl.pause();
	vidEl.currentTime = 0;
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



async function open (_item, clickedEl) {
	resetVideo();
	item = _item;
	let full;

	if (item.type === 'video') await tick().then(createVideo);
	else {
		full = new Image();
		full.onload = imgOnLoad;
	}

	if (!isOpen) {
		mediaItemElements = document.querySelectorAll('.main .media-item');
		el.style.display = 'flex';
		el.style.transformOrigin = getBoxCenter(clickedEl);
		document.documentElement.style.overflow = 'hidden';  // hide scrollbars
		await animate(el, thumbProps, fullScreenProps);
	}
	if (full) full.src = _item.path;
	isOpen = true;
	if (item.type === 'video') tick().then(() => vidEl.focus());
}

async function close (e) {
	if (e?.target?.closest('video')) return;
	if (imgEl) imgEl.src = item.thumb; // for smoother animation
	await resetVideo();
	const targetElement = Array.from(mediaItemElements).find(i => i.id === item.path);
	el.style.transformOrigin =  getBoxCenter(targetElement);
	await animate(el, fullScreenProps, thumbProps);
	el.style.display = 'none';
	document.documentElement.style.overflow = '';
	if (imgEl) imgEl.src = '#';
	isOpen = false;
	tick().then(() => targetElement.focus());
}
</script>
