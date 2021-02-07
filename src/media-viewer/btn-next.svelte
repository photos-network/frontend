<a class="icon-btn btn-next" title="Next"
	href="#{$activeSection}/preview/{nextItemId}"
	hidden="{hidden}"
	bind:this="{el}">
		<Icon name="chevronRight"/>
</a>

<script>
import { onMount } from 'svelte';
import { EVENT, activeSection, activeID, items } from '../lib';
import Icon from '../icon';
let el;
let hidden = false;
let nextItemId = '';

onMount(() => {
	activeID.subscribe(onItemChange);
	EVENT.on(EVENT.item.next, next);
});

function next () {
	if (el) location.hash = el.hash.substr(1);
}

async function onItemChange (id) {
	if (!$items?.length) await items.rendered();
	const mediaItemElements = document.querySelectorAll('.main .media-item');
	if (mediaItemElements?.length) {
		const idx = Array.from(mediaItemElements).findIndex(i => i.id === id);
		if (idx < mediaItemElements?.length - 1) {
			const nextItemIdx = (idx < mediaItemElements.length ? idx + 1 : idx);
			nextItemId = mediaItemElements[nextItemIdx].id;
			hidden = false;
		}
		else hidden = true;
	}
	else hidden = true;
}
</script>
