<div class="media-viewer" bind:this="{el}" data-item-path="{item.path}">
	<Menu {isOpen} />
	<div class="media-viewer-img-wrap">
		<BtnPrev />
		<svelte:component this={component} item="{item}"/>
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
import Video from './video';
import Image from './image';
import { EVENT, animate, getBoxCenter, items } from '../lib';

let item = { src: '', name: '', type: 'photo' };
$:component = item.type === 'video' ? Video : Image;

let el;
let isOpen = false;
const thumbProps = { transform: 'scale(0.1)', opacity: 0 };
const fullScreenProps = { transform: 'scale(1)', opacity: 1 };


onMount(() => {
	EVENT.on(EVENT.nav.afterChange, onNavChange);
});


async function onNavChange (nav) {
	if (!$items.length) await items.rendered();
	if (isOpen && nav.action !== 'preview') return close();
	if (nav.action === 'preview' && nav.id) {
		const _item = $items.find(i => i.id === nav.id);
		return open(_item);
	}
}


async function open (_item) {
	item = _item;
	if (!isOpen) await toggle(true);
	isOpen = true;
}


async function close () {
	await toggle(false);
	isOpen = false;
}



/**
 * Open/Close animation
 * @param opening {boolean}
 */
async function toggle (opening) {
	const targetElement = document.querySelector(`.main .media-item[id=${item.id}`);
	el.style.transformOrigin = getBoxCenter(targetElement);

	if (opening) {
		document.documentElement.style.overflow = 'hidden';  // hide scrollbars
		el.style.display = 'flex';
		await animate(el, thumbProps, fullScreenProps);
	}
	else {
		await animate(el, fullScreenProps, thumbProps);
		el.style.display = 'none';
		document.documentElement.style.overflow = '';
		tick().then(() => targetElement.focus());
	}
}



</script>
