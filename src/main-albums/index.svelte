{#if $items}
	<MediaGroup group="{group}" />
{/if}

<script>
import MediaGroup from '../media-group';
import { items } from '../lib';

$:group = { items: $items };
</script>
