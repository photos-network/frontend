<nav>
	<MenuLine />
	<a class="icon-btn btn-menu" title="Menu" href="#menu" on:click|preventDefault="{() => alert('dragons')}">
		<Icon name="dots"/>
	</a>
	<div class="menu">
		{#each menuItems as item}
			<a class="icon-btn" class:active="{$activeSection === item.id}"
			title="{item.title}" href="#{item.id}"><Icon name="{item.icon}" />
		</a>
		{/each}
	</div>
</nav>

<script>
import MenuLine from './menu-line';
import { activeSection } from '../lib';
import Icon from '../icon';

const menuItems = [
	{ id: 'timeline', title: 'Timeline', icon: 'calendar' },
	{ id: 'albums',   title: 'Albums',   icon: 'book'     },
	{ id: 'folders',  title: 'Folders',  icon: 'folder'   },
	{ id: 'map',      title: 'Map',      icon: 'map'      },
	{ id: 'shared',   title: 'Shared',   icon: 'shared'   },
];


</script>
