<Header />
<Main />
<MediaViewer />
<Router />

<script>
import { onMount } from 'svelte';
import Header from '../header';
import Main from '../main';
import MediaViewer from '../media-viewer';
import Router from '../router';
import { EVENT } from '../lib';

let resizeTimer;

onMount(() => {
	document.addEventListener('click', e => EVENT.fire(EVENT.document.clicked, e));
	document.addEventListener('keydown', e => EVENT.fire(EVENT.document.keydown, e), true);

	window.addEventListener('resize', onResize);

	EVENT.fire(EVENT.app.started);
});

function onResize () {
	if (resizeTimer) clearTimeout(resizeTimer);
	resizeTimer = setTimeout(onResizeEnd, 200); // debounce
}

function onResizeEnd () {
	EVENT.fire(EVENT.window.resize);
}
</script>
