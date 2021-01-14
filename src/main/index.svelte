<main class="main">
	{#if $initialised}
		{#if $groups}
			{#each $groups as group}
				<MediaGroup group="{group}" />
			{/each}
		{:else}
			Add some photos
		{/if}
	{:else}
		Loading...
	{/if}
</main>

<script>
import MediaGroup from '../media-group';
import { initialised, groups } from '../lib';

</script>
