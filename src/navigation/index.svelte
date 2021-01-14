<svelte:window on:popstate={onpopstate}/>

<nav>
	<a class="icon-btn" title="Timeline" class:active="{active === 'timeline'}" href="#timeline">
		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-calendar-event" width="32" height="32" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<rect x="4" y="5" width="16" height="16" rx="2" />
			<line x1="16" y1="3" x2="16" y2="7" />
			<line x1="8" y1="3" x2="8" y2="7" />
			<line x1="4" y1="11" x2="20" y2="11" />
			<rect x="8" y="15" width="2" height="2" />
		  </svg>
	</a>

	<a class="icon-btn" title="Albums" class:active="{active === 'albums'}" href="#albums">
		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-book" width="32" height="32" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<path d="M3 19a9 9 0 0 1 9 0a9 9 0 0 1 9 0" />
			<path d="M3 6a9 9 0 0 1 9 0a9 9 0 0 1 9 0" />
			<line x1="3" y1="6" x2="3" y2="19" />
			<line x1="12" y1="6" x2="12" y2="19" />
			<line x1="21" y1="6" x2="21" y2="19" />
		  </svg>
	</a>

	<a class="icon-btn" title="Folders" class:active="{active === 'folders'}" href="#folders">
		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-folder" width="32" height="32" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<path d="M5 4h4l3 3h7a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2" />
		  </svg>
	</a>

	<a class="icon-btn" title="Map" class:active="{active === 'map'}" href="#map">
		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-map-2" width="32" height="32" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<line x1="18" y1="6" x2="18" y2="6.01" />
			<path d="M18 13l-3.5 -5a4 4 0 1 1 7 0l-3.5 5" />
			<polyline points="10.5 4.75 9 4 3 7 3 20 9 17 15 20 21 17 21 15" />
			<line x1="9" y1="4" x2="9" y2="17" />
			<line x1="15" y1="15" x2="15" y2="20" />
		  </svg>
	</a>

	<a class="icon-btn" title="Shared" class:active="{active === 'shared'}" href="#shared">
		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-share" width="32" height="32" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<circle cx="6" cy="12" r="3" />
			<circle cx="18" cy="6" r="3" />
			<circle cx="18" cy="18" r="3" />
			<line x1="8.7" y1="10.7" x2="15.3" y2="7.3" />
			<line x1="8.7" y1="13.3" x2="15.3" y2="16.7" />
		  </svg>
	</a>
</nav>

<script>

let active = 'timeline';

function onpopstate () {
	active = location.hash?.substr(1) || 'timeline';
}

onpopstate();

</script>
