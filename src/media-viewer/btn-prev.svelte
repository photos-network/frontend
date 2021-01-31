<a href="#prev" class="icon-btn btn-prev" title="Previous"
	on:click|preventDefault|stopPropagation="{prev}">
	<Icon name="chevronLeft"/>
</a>

<script>
import { EVENT } from '../lib';
import Icon from '../icon';

function prev () {
	EVENT.fire(EVENT.item.prev);
}

</script>
