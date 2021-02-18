const ANIMATION_SPEED = 300;

// native js animation
function animate (el, from, to, _options = {}) {
	const dflt = { duration: ANIMATION_SPEED, easing: 'ease-out', fill: 'forwards' };
	const opts = Object.assign({}, dflt, _options);

	return new Promise(resolve => {
		requestAnimationFrame(() => {
			const anim = el.animate([from, to], opts);
			anim.oncancel = resolve;
			anim.onfinish = resolve;
		});
	});
}


function clone (obj) {
	return JSON.parse(JSON.stringify(obj));
}


function copyToClipboard (text) {
	const inp = document.createElement('INPUT');
	inp.value = text;
	inp.style = 'position: fixed; left: -1000px; top: -1000px;';
	document.body.appendChild(inp);
	try {
		inp.select();
		inp.setSelectionRange(0, 99999); // For mobile devices
		document.execCommand('copy');
	}
	catch (e) {
		console.error(e);
	}
	setTimeout(() => inp.remove());
}


function fuzzy (hay = '', s = '') {
	hay = hay.toLowerCase();
	let n = -1;
	for (let l of s) if (!~(n = hay.indexOf(l, n + 1))) return false;
	return true;
}


function getBoxCenter (el) {
	const docEl = document.documentElement;
	const {left, top, width, height} = el?.getBoundingClientRect() ||
		{ left: docEl.offsetWidth / 2, top: docEl.offsetHeight / 2, width: 0, height: 0 };
	return `${left + width / 2}px ${top + height / 2}px`;
}


function inView (elem, cb = () => {}) {
	const options = { rootMargin: '200px', threshold: 1.0 };
	const observer = new IntersectionObserver(([entry]) => {
		if (entry.intersectionRatio > 0.9) {
			observer.unobserve(entry.target);
			cb();
		}
	}, options);
	observer.observe(elem);
}

function sleep (dur) {
	return new Promise(resolve => {
		setTimeout(() => resolve(), dur || 0);
	});
}

function sortBy (items, field = 'date_taken') {
	return items.sort((a, b) => a[field] - b[field]);
}


function formatBytes (bytes, decimals = 2) {
	if (bytes === 0) return '0 Bytes';
	const k = 1024;
	const dm = decimals < 0 ? 0 : decimals;
	const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

function guid () {
	return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, c => {
		const r = Math.random() * 16 | 0;
		const v = c == 'x' ? r : (r & 0x3 | 0x8);
		return v.toString(16);
	});
}

function isVideoType (type) {
	return type.includes('video');
}


export {
	animate,
	clone,
	copyToClipboard,
	fuzzy,
	getBoxCenter,
	inView,
	sleep,
	sortBy,
	formatBytes,
	guid,
	isVideoType,
};
