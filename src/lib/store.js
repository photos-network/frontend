import { writable } from 'svelte/store';
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

EVENT.on(EVENT.app.started, () => {
	initialised.set(true);
	items.load();
});
