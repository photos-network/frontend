<main class="main">
	{#if $initialised}
		<svelte:component this={section}/>
	{:else}
		Loading...
	{/if}
</main>

<script>
import { initialised, activeSection } from '../lib';
import timeline from '../main-timeline';
import albums from '../main-albums';

const sections = {
	timeline,
	albums
};

$:section = sections[$activeSection];

</script>
