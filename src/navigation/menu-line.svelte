<div class="menu-line" class:initial bind:this="{el}"></div>

<script>
import { onMount } from 'svelte';
import { activeSection } from '../lib';
let initial = true;
let el;

onMount(() => {
	activeSection.subscribe(onchange);
});


function onchange (v) {
	const menuItem = el.parentNode.querySelector(`a[href="#${v}"]`);
	el.style.transform = `translateX(${menuItem.offsetLeft}px)`;
	if (initial) setTimeout(() => initial = false, 300);
}

</script>
