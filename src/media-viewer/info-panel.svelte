<div class="info-panel" transition:fly="{{ x: 310, duration: 3000 }}" bind:this="{el}">
	<div class="info-panel-inner">
		Name: {item.name || ''}<br>
		Description: {item.description || ''}<br>
		Type: {item.type || ''}<br>
		Date: {item.date_taken || ''}<br>
	</div>
</div>



<script>
import { onMount } from 'svelte';
import { fly } from 'svelte/transition';
import { EVENT, animate } from '../lib';
export let item = {};
let isOpen = false;
let el;

onMount(() => {
	EVENT.on(EVENT.info.toggle, toggle);
});


function toggle (force) {
	if (force === true) open();
	else if (force === false) close();
	else {
		if (isOpen) close();
		else open();
	}
}

async function open () {
	await animate(el, { width: 0 }, { width: '300px' });
	isOpen = true;

}

async function close () {
	await animate(el, { width: '300px' }, { width: 0 });
	isOpen = false;

}

</script>
