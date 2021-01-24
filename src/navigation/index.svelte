<svelte:window on:popstate={onpopstate}/>

<nav>
	<MenuLine />
	<a class="icon-btn" title="Timeline" class:active="{$activeSection === 'timeline'}" href="#timeline">
		<ICON name="calendar" /></a>
	<a class="icon-btn" title="Albums" class:active="{$activeSection === 'albums'}" href="#albums">
		<ICON name="book" /></a>
	<a class="icon-btn" title="Folders" class:active="{$activeSection === 'folders'}" href="#folders">
		<ICON name="folder" /></a>
	<a class="icon-btn" title="Map" class:active="{$activeSection === 'map'}" href="#map">
		<ICON name="map" /></a>
	<a class="icon-btn" title="Shared" class:active="{$activeSection === 'shared'}" href="#shared">
		  <ICON name="shared" /></a>
</nav>

<script>
import MenuLine from './menu-line';
import { activeSection } from '../lib';
import ICON from '../icon';

function onpopstate () {
	$activeSection = location.hash?.substr(1) || 'timeline';
}

onpopstate();
</script>
