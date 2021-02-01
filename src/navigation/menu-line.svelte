<div class="menu-line" class:initial bind:this="{el}"></div>

<script>
import { onMount } from 'svelte';
import { activeSection, EVENT } from '../lib';
let initial = true;
let el;

onMount(() => {
	activeSection.subscribe(onchange);
	EVENT.on(EVENT.window.resize, () => onchange());
});


function onchange (v) {
	if (!v) v = $activeSection;
	const menuItem = el.parentNode.querySelector(`a[href="#${v}"]`);
	el.style.transform = `translateX(${menuItem.offsetLeft}px)`;
	if (initial) setTimeout(() => initial = false, 300);
}

</script>
