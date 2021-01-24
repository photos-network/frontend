<div class="media-viewer-menu">
	<div class="flex-spacer"></div>

	<a href="#share" class="icon-btn btn-shared" title="Share"
		on:click|preventDefault|stopPropagation="{share}">
		<ICON name="shared"/>
	</a>
	<a href="#info" class="icon-btn btn-info" title="Toggle Info Panel"
		on:click|preventDefault|stopPropagation="{info}">
		<ICON name="info"/>
	</a>
	<a href="#delete" class="icon-btn btn-delete" title="Delete"
		on:click|preventDefault|stopPropagation="{del}">
		<ICON name="trash"/>
	</a>
	<a href="#close" class="icon-btn btn-close" title="Close Viewer"
		on:click|preventDefault|stopPropagation="{close}">
		<ICON name="close"/>
	</a>
</div>



<script>
import { onMount } from 'svelte';
import { EVENT } from '../lib';
import ICON from '../icon';

export let isOpen;
const keyMap = {
	ArrowLeft: () => EVENT.fire(EVENT.item.prev),
	ArrowRight: () => EVENT.fire(EVENT.item.next),
	Escape: () => EVENT.fire(EVENT.item.close),
};

onMount(() => {
	EVENT.on(EVENT.document.keydown, onKeydown);
});


function onKeydown (e) {
	if (!isOpen) return;
	if (typeof keyMap[e.key] === 'function') keyMap[e.key](e);
}

function close () {
	EVENT.fire(EVENT.item.close);
}

function share () {
	// EVENT.fire(EVENT.info.toggle);
}

function info () {
	EVENT.fire(EVENT.info.toggle);
}

function del () {
	console.log('delete');
}

</script>
