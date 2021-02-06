<div class="media-viewer-menu">
	<div class="flex-spacer"></div>

	<a href="#share" class="icon-btn btn-shared" title="Share"
		on:click|preventDefault|stopPropagation="{share}">
		<Icon name="shared"/>
	</a>
	<a href="#info" class="icon-btn btn-info" title="Toggle Info Panel"
		on:click|preventDefault|stopPropagation="{info}">
		<Icon name="info"/>
	</a>
	<a href="#delete" class="icon-btn btn-delete" title="Delete"
		on:click|preventDefault|stopPropagation="{del}">
		<Icon name="trash"/>
	</a>
	<a href="#{$activeSection}" class="icon-btn btn-close" title="Close Viewer">
		<Icon name="close"/>
	</a>
</div>



<script>
import { onMount } from 'svelte';
import { EVENT, activeSection } from '../lib';
import Icon from '../icon';

export let isOpen;
const keyMap = {
	ArrowLeft: () => EVENT.fire(EVENT.item.prev),
	ArrowRight: () => EVENT.fire(EVENT.item.next),
	Escape: () => location.hash = $activeSection,
};

onMount(() => {
	EVENT.on(EVENT.document.keydown, onKeydown);
});


function onKeydown (e) {
	if (!isOpen) return;
	if (typeof keyMap[e.key] === 'function') keyMap[e.key](e);
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
