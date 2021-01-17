// native js animation
function animate (el, from, to, _options = {}) {
	const dflt = {duration: 200, easing: 'ease-out', fill: 'forwards'};
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


function getBox (el) {
	const docEl = document.documentElement;
	const {left, top, width, height} = el?.getBoundingClientRect() ||
		{ left: docEl.offsetWidth / 2, top: docEl.offsetHeight / 2, width: 0, height: 0 };
	return {
		left: left + 'px',
		top: top + 'px',
		width: width + 'px',
		height: height + 'px',
	};
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


function sortBy (items, field = 'date_taken') {
	return items.sort((a, b) => a[field] - b[field]);
}


export {
	animate,
	clone,
	copyToClipboard,
	fuzzy,
	getBox,
	inView,
	sortBy,
};
