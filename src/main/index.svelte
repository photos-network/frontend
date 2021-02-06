<main class="main" class:loading>
	{#if $initialised}
		<svelte:component this={section}/>
	{:else}
		Loading...
	{/if}
</main>

<script>
import { onMount } from 'svelte';
import { EVENT, initialised, sleep } from '../lib';
import timeline from '../main-timeline';
import albums from '../main-albums';

let loading = false, section;

const sections = {
	timeline,
	albums
};


onMount(() => {
	EVENT.on(EVENT.nav.section.beforeChange, startSectionChange);
	EVENT.on(EVENT.nav.section.afterChange, finishSectionChange);
});


function startSectionChange () {
	loading = true;
}


async function finishSectionChange (sect) {
	// allow fadeout anim to finish before rendering the new section
	await sleep(200);
	// render new section when opacity = 0
	section = sections[sect];
	loading = false;
}

</script>
