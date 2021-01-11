import {on, off, fire} from './pubsub';
export const EVENT = {
	on,
	off,
	fire,

	app: {
		started: 'app-started'
	},
	document: {
		clicked: 'document-clicked',
	},

};
