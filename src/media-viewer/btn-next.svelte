<a href="#next" class="icon-btn btn-next" title="Next"
	on:click|preventDefault|stopPropagation="{next}">
	<ICON name="chevronRight"/>
</a>


<script>
import { EVENT } from '../lib';
import ICON from '../icon';

function next () {
	EVENT.fire(EVENT.item.next);
}

</script>
