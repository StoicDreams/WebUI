(function webuiSetup(){
	'strict'
	console.log('Web UI Setup')
	function getEl(sel, mswait) {
		return new Promise((resolve, reject) => {
			let el = null;
			const start = Date.now();
			(function check(){
				el = document.querySelector(sel);
				console.log(`Query Selector $sel}`, el);
				if (el) {
					resolve(el);
					return;
				}
				const c = Date.now();
				if (c - start > mswait) {
					reject(`Element ${sel} not found.`);
					return;
				}
				setTimeout(check,10);
			})();
		});
	}
	function handleNav(target) {
		if (!target.href) {return false;}
		if (target.href[0] === '/'
			|| target.href.substr(0, location.origin.length).toLowerCase() === location.origin.toLowerCase()) {
			//history.pushState(null, null, target.href);
			location.href = target.href;
			return true;
		}
		if (target.href.substr(0,4) === 'http') {
			window.open(target.href, '_blank');
			return true;
		}
		return false;
	}
	getEl('#app', 30000).then(el => {
		el.addEventListener('click', ev => {
			if (!ev.target) { return; }
			if (handleNav(ev.target)) {
				ev.preventDefault();
				ev.stopPropagation();
				return false;
			}
		});
	});
})();

