
function animate (el, from, to, _options = {}) {
	const dflt = {duration: 300, easing: 'ease-out', fill: 'forwards'};
	const opts = Object.assign({}, dflt, _options);

	return new Promise(resolve => {
		const anim = el.animate([from, to], opts);
		anim.oncancel = resolve;
		anim.onfinish = resolve;
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


export {
	animate,
	clone,
	copyToClipboard,
	fuzzy,
};
