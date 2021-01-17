import { writable, derived } from 'svelte/store';
import { EVENT } from './event';


export const initialised = writable(false);

function ItemStore () {
	const { subscribe, set } = writable([]);

	function load () {
		return fetch('items.json')
			.then(res => res.json())
			.then(set);
	}

	function upload () {
		// dragons, etc.
	}

	return {
		subscribe,
		load,
		upload,
		reset: () => set([])
	};
}

export const items = ItemStore([]);

export const groups = derived(items, $items => {
	const grps = {};
	$items.forEach(item => {
		const title = item.date_taken;
		grps[title] = grps[title] || { title, items: [] };
		grps[title].items.push(item);
	});
	return Object.values(grps) || [];
});


EVENT.on(EVENT.app.started, () => {
	initialised.set(true);
	items.load();
});
