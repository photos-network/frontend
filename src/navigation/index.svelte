<svelte:window on:popstate={onpopstate}/>

<nav>
	<MenuLine />
	{#each menuItems as item}
		<a class="icon-btn" class:active="{$activeSection === item.id}"
			title="{item.title}" href="#{item.id}"><Icon name="{item.icon}" />
		</a>
	{/each}
</nav>

<script>
import MenuLine from './menu-line';
import { activeSection, EVENT } from '../lib';
import Icon from '../icon';
import { onMount, tick } from 'svelte';
const menuItems = [
	{ id: 'timeline', title: 'Timeline', icon: 'calendar' },
	{ id: 'albums',   title: 'Albums',   icon: 'book'     },
	{ id: 'folders',  title: 'Folders',  icon: 'folder'   },
	{ id: 'map',      title: 'Map',      icon: 'map'      },
	{ id: 'shared',   title: 'Shared',   icon: 'shared'   },
];

function onpopstate () {
	EVENT.fire(EVENT.app.beforeSectionChange);
	$activeSection = location.hash?.substr(1) || 'timeline';
	EVENT.fire(EVENT.app.afterSectionChange, $activeSection);
}

onMount(() => {
	tick().then(onpopstate);
});
</script>
