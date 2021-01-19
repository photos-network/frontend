<a href="#next" class="icon-btn btn-next" title="Next"
	on:click|preventDefault|stopPropagation="{next}">
	<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-chevron-right" width="32" height="32" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
		<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
		<polyline points="9 6 15 12 9 18" />
	</svg>
</a>


<script>
import { EVENT } from '../lib';

function next () {
	EVENT.fire(EVENT.item.next);
}

</script>
