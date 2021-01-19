<div class="media-viewer-menu">
	<div class="flex-spacer"></div>

	<a href="#share" class="icon-btn btn-shared" title="Share"
		on:click|preventDefault|stopPropagation="{info}">
		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-share" width="32" height="32" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<circle cx="6" cy="12" r="3" />
			<circle cx="18" cy="6" r="3" />
			<circle cx="18" cy="18" r="3" />
			<line x1="8.7" y1="10.7" x2="15.3" y2="7.3" />
			<line x1="8.7" y1="13.3" x2="15.3" y2="16.7" />
		</svg>
	</a>
	<a href="#info" class="icon-btn btn-info" title="Toggle Info Panel"
		on:click|preventDefault|stopPropagation="{info}">
		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-info-circle" width="32" height="32" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<circle cx="12" cy="12" r="9" />
			<line x1="12" y1="8" x2="12.01" y2="8" />
			<polyline points="11 12 12 12 12 16 13 16" />
		</svg>
	</a>
	<a href="#delete" class="icon-btn btn-delete" title="Delete"
		on:click|preventDefault|stopPropagation="{del}">
		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-trash" width="32" height="32" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<line x1="4" y1="7" x2="20" y2="7" />
			<line x1="10" y1="11" x2="10" y2="17" />
			<line x1="14" y1="11" x2="14" y2="17" />
			<path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12" />
			<path d="M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" />
		</svg>
	</a>
	<a href="#close" class="icon-btn btn-close" title="Close Viewer"
		on:click|preventDefault|stopPropagation="{close}">
		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-x" width="84" height="84" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<line x1="18" y1="6" x2="6" y2="18" />
			<line x1="6" y1="6" x2="18" y2="18" />
		</svg>
	</a>
</div>



<script>
import { onMount } from 'svelte';
import { EVENT } from '../lib';
export let isOpen;
const keyMap = {
	ArrowLeft: () => EVENT.fire(EVENT.item.prev),
	ArrowRight: () => EVENT.fire(EVENT.item.next),
	Escape: () => EVENT.fire(EVENT.item.close),
};

onMount(() => {
	document.addEventListener('keydown', onKeydown, true);
});


function onKeydown (e) {
	if (!isOpen) return;
	if (typeof keyMap[e.key] === 'function') keyMap[e.key](e);
}

function close () {
	EVENT.fire(EVENT.item.close);
}

function info () {
	EVENT.fire(EVENT.info.toggle);
}

function del () {
	console.log('delete');
}

</script>
