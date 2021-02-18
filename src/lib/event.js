import {on, off, fire} from './pubsub';
export const EVENT = {
	on,
	off,
	fire,

	app: {
		started: 'app-started',
	},
	nav: {
		beforeChange: 'nav-before-change',
		afterChange: 'nav-after-change',
		section: {
			beforeChange: 'nav-section-after-change',
			afterChange: 'nav-section-before-change',
		},
		action: {
			beforeChange: 'nav-action-after-change',
			afterChange: 'nav-action-before-change',
		},
		id: {
			beforeChange: 'nav-id-after-change',
			afterChange: 'nav-id-before-change',
		}
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
	uploader: {
		browse: 'uploader-browse',
	},
	window: {
		resize: 'window-resize',
	},

};
