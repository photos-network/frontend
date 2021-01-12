<Header />
<Main />

<script>
import { onMount } from 'svelte';
import Header from '../header';
import Main from '../main';
import { EVENT } from '../lib';

onMount(() => {
	document.addEventListener('click', e => EVENT.fire(EVENT.document.clicked, e));
	EVENT.fire(EVENT.app.started);
});
</script>
