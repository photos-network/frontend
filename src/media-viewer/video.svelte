<div class="video-wrapper" bind:this="{wrapperEl}"></div>

<script>
import { afterUpdate, onDestroy, tick } from 'svelte';

export let item = { src: '', name: '', type: 'video' };
let el, wrapperEl;


afterUpdate(() => {
	resetVideo();
	tick().then(createVideo);
});

onDestroy(() => {
	resetVideo();
});


function resetVideo () {
	if (!el) return;
	el.pause();
	el.remove();
}

/**
 * This manual process is required as in a html template
 * svelte would only replace what has changed, so src & poster attribute values
 * however, to fully "reset" a video tag (to show the poster again) it must be
 * removed and re-added to the DOM
 */
function createVideo () {
	el = document.createElement('VIDEO');
	el.poster = item.thumb;
	el.controls = 'controls';
	wrapperEl.appendChild(el);
	tick().then(() => {
		const src = document.createElement('SOURCE');
		src.src = item.path;
		el.appendChild(src);
		tick().then(() => el.focus());
	});
}

</script>
