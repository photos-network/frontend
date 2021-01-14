import {on, off, fire} from './pubsub';
export const EVENT = {
	on,
	off,
	fire,

	app: {
		started: 'app-started'
	},
	item: {
		view: 'item-view'
	},
	document: {
		clicked: 'document-clicked',
	},

};
