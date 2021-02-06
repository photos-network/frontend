<div class="menu-line" class:initial bind:this="{el}"></div>

<script>
import { onMount } from 'svelte';
import { activeSection, EVENT } from '../lib';
let initial = true;
let el;

onMount(() => {
	EVENT.on(EVENT.nav.afterChange, ({section}) => onchange(section));
	EVENT.on(EVENT.window.resize, () => onchange());
});


function onchange (v = $activeSection) {
	const menuItem = el.parentNode.querySelector(`a[href="#${v}"]`);
	if (menuItem) el.style.transform = `translateX(${menuItem.offsetLeft}px)`;
	if (initial) setTimeout(() => initial = false, 300);
}

</script>
