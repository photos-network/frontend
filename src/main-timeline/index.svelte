{#if $groups}
	{#each $groups as group}
		<MediaGroup group="{group}" />
	{/each}
{/if}

<script>
import MediaGroup from '../media-group';
import { groups } from '../lib';

</script>
