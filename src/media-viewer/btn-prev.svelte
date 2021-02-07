<a class="icon-btn btn-prev" title="Previous"
	href="#{$activeSection}/preview/{prevItemId}"
	hidden="{hidden}"
	bind:this="{el}">
		<Icon name="chevronLeft"/>
</a>

<script>
import { onMount } from 'svelte';
import { EVENT, activeSection, activeID, items } from '../lib';
import Icon from '../icon';
let el;
let hidden = false;
let prevItemId = '';

onMount(() => {
	activeID.subscribe(onItemChange);
	EVENT.on(EVENT.item.prev, prev);
});

function prev () {
	if (el) location.hash = el.hash.substr(1);
}

async function onItemChange (id) {
	if (!$items?.length) await items.rendered();
	const mediaItemElements = document.querySelectorAll('.main .media-item');
	if (mediaItemElements?.length) {
		const idx = Array.from(mediaItemElements).findIndex(i => i.id === id);
		if (idx > 0) {
			const prevItemIdx = (idx > 0 ? idx - 1 : idx);
			prevItemId = mediaItemElements[prevItemIdx].id;
			hidden = false;
		}
		else hidden = true;
	}
	else hidden = true;
}

</script>
