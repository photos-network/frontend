{#if $filteredItems?.length}
	<MediaGroup group="{group}" />
{/if}

<script>
import MediaGroup from '../media-group';
import { filteredItems } from '../lib';

$:group = { items: $filteredItems };
</script>
