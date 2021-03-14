{#if group.title}
	<h2 class="media-group-title">{group.title}</h2>
{/if}
<div class="media-group">
	{#each group.items as item (item.id)}
		<MediaItem src="{item}" />
	{/each}
</div>


<script>
import MediaItem from '../media-item';

export let group = { title: '', items: [] };

</script>
