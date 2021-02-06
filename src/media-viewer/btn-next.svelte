<a class="icon-btn btn-next" title="Next"
	href="#{$activeSection}/preview/{nextItemId}"
	bind:this="{el}">
		<Icon name="chevronRight"/>
</a>

<script>
import { onMount } from 'svelte';
import { EVENT, activeSection, activeID } from '../lib';
import Icon from '../icon';
let el;
let nextItemId = '';

onMount(() => {
	activeID.subscribe(onItemChange);
	EVENT.on(EVENT.item.next, next);
});

function next () {
	if (el) location.hash = el.hash.substr(1);
}

function onItemChange (id) {
	const mediaItemElements = document.querySelectorAll('.main .media-item');
	if (!mediaItemElements?.length) return 0;
	const idx = Array.from(mediaItemElements).findIndex(i => i.id === id);

	if (idx >= mediaItemElements?.length - 1) return;
	const nextItemIdx = (idx < mediaItemElements.length ? idx + 1 : idx);
	nextItemId = mediaItemElements[nextItemIdx].id;
}
</script>
