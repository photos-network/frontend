<a class="icon-btn btn-prev" title="Previous"
	href="#{$activeSection}/preview/{prevItemId}"
	bind:this="{el}">
		<Icon name="chevronLeft"/>
</a>

<script>
import { onMount } from 'svelte';
import { EVENT, activeSection, activeID } from '../lib';
import Icon from '../icon';
let el;
let prevItemId = '';

onMount(() => {
	activeID.subscribe(onItemChange);
	EVENT.on(EVENT.item.prev, prev);
});

function prev () {
	if (el) location.hash = el.hash.substr(1);
}

function onItemChange (id) {
	const mediaItemElements = document.querySelectorAll('.main .media-item');
	if (!mediaItemElements?.length) return 0;
	const idx = Array.from(mediaItemElements).findIndex(i => i.id === id);

	if (idx <= 0) return;
	const prevItemIdx = (idx > 0 ? idx - 1 : idx);
	prevItemId = mediaItemElements[prevItemIdx].id;
}

</script>
