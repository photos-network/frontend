<svelte:window on:popstate={onpopstate}/>

<nav>
	<MenuLine />
	{#each menuItems as item}
		<a class="icon-btn" class:active="{$activeSection === item.id}"
			title="{item.title}" href="#{item.id}"><ICON name="{item.icon}" />
		</a>
	{/each}
</nav>

<script>
import MenuLine from './menu-line';
import { activeSection } from '../lib';
import ICON from '../icon';
const menuItems = [
	{ id: 'timeline', title: 'Timeline', icon: 'calendar' },
	{ id: 'albums',   title: 'Albums',   icon: 'book'     },
	{ id: 'folders',  title: 'Folders',  icon: 'folder'   },
	{ id: 'map',      title: 'Map',      icon: 'map'      },
	{ id: 'shared',   title: 'Shared',   icon: 'shared'   },
];

function onpopstate () {
	$activeSection = location.hash?.substr(1) || 'timeline';
}

onpopstate();
</script>
