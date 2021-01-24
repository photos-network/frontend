<a href="#prev" class="icon-btn btn-prev" title="Previous"
	on:click|preventDefault|stopPropagation="{prev}">
	<ICON name="chevronLeft"/>
</a>

<script>
import { EVENT } from '../lib';
import ICON from '../icon';

function prev () {
	EVENT.fire(EVENT.item.prev);
}

</script>
