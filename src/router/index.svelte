<svelte:window on:popstate={onpopstate}/>

<script>
import { activeSection, activeAction, activeID, EVENT } from '../lib';
import { onMount, tick } from 'svelte';

onMount(() => {
	tick().then(onpopstate);
});

function onpopstate () {
	if (!location.hash) return location.hash = 'timeline';
	const [ section, action, id ] = location.hash?.substr(1).split('/');

	if ($activeSection === section && $activeAction === action && $activeID === id) return;

	EVENT.fire(EVENT.nav.beforeChange);
	if ($activeSection !== section) {
		EVENT.fire(EVENT.nav.section.beforeChange);
		activeSection.set(section);
		EVENT.fire(EVENT.nav.section.afterChange, section);
	}
	if ($activeAction !== action) {
		EVENT.fire(EVENT.nav.action.beforeChange);
		activeAction.set(action);
		EVENT.fire(EVENT.nav.action.afterChange, action);
	}
	if ($activeID !== id) {
		EVENT.fire(EVENT.nav.id.beforeChange);
		activeID.set(id);
		EVENT.fire(EVENT.nav.id.afterChange, id);
	}
	EVENT.fire(EVENT.nav.afterChange, { section, action, id });
}

</script>
