import {on, off, fire} from './pubsub';
export const EVENT = {
	on,
	off,
	fire,

	app: {
		started: 'app-started',
		beforeSectionChange: 'before-section-change',
		afterSectionChange: 'after-section-change',
	},
	item: {
		view: 'item-view',
		close: 'item-close',
		prev: 'item-prev',
		next: 'item-next',
	},
	info: {
		toggle: 'info-panel-toggle',
	},
	document: {
		clicked: 'document-clicked',
		keydown: 'document-keydown',
	},
	window: {
		resize: 'window-resize',
	},

};
