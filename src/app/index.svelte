<Header />
<Main />
<MediaViewer />
<script>
import { onMount } from 'svelte';
import Header from '../header';
import Main from '../main';
import MediaViewer from '../media-viewer';
import { EVENT } from '../lib';

onMount(() => {
	document.addEventListener('click', e => EVENT.fire(EVENT.document.clicked, e));
	document.addEventListener('keydown', e => EVENT.fire(EVENT.document.keydown, e), true);
	EVENT.fire(EVENT.app.started);
});
</script>
