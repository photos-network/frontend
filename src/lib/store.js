import { writable, derived } from 'svelte/store';
import { EVENT } from './event';
import { sleep } from './utils';

export const activeSection = writable('');
export const activeAction = writable('');
export const activeID = writable('');

export const initialised = writable(false);


EVENT.on(EVENT.app.started, () => {
	initialised.set(true);
	items.load();
});

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

	/**
	 * This resolves once the items "are rendered"
	 */
	async function rendered () {
		return new Promise(resolve => {
			let unsub;
			unsub = items.subscribe(async () => {
				if (typeof unsub !== 'function') return;
				unsub();
				await sleep(500); // ugly but required
				resolve();
			});
		});
	}

	return {
		subscribe,
		load,
		upload,
		rendered,
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
