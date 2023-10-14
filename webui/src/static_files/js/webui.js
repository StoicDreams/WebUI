(function webuiSetup() {
    "use strict"
    function getEl(sel, mswait) {
        return new Promise((resolve, reject) => {
            let el = null;
            const start = Date.now();
            (function check() {
                el = document.querySelector(sel);
                if (el) {
                    resolve(el);
                    return;
                }
                const c = Date.now();
                if (c - start > mswait) {
                    reject(`Element ${sel} not found.`);
                    return;
                }
                setTimeout(check, 10);
            })();
        });
    }
    function getMatchByKey(target, key) {
        let el = target;
        let i = 0;
        while (el && i++ < 10) {
            if (el[key]) { return el; }
            el = el.parentNode;
        }
        return undefined;
    }
    const styles = document.createElement('style');
    function set_body_class(winWidth) {
        // Flag general width by class
        let w = winWidth > 3800 ? 'w-4k'
            : winWidth > 3400 ? 'w-wqhd'
                : winWidth > 2500 ? 'w-qhd'
                    : winWidth > 1900 ? 'w-fhd'
                        : winWidth > 1500 ? 'w-hdp'
                            : winWidth > 1300 ? 'w-hd'
                                : winWidth > 500 ? 'w-tab'
                                    : 'w-mob'
            ;
        document.body.className = `${w}`;
    }
    async function applyDynamicStyleRules() {
        let w = window;
        let h = await getEl('#app > header', 1) || { clientHeight: 0 };
        let m = await getEl('#app > main', 1) || { clientHeight: 0, clientWidth: 0 };
        let f = await getEl('#app > footer', 1) || { clientHeight: 0 };
        styles.innerHTML = `
:root {
    --window-width: ${w.innerWidth}px;
    --window-height: ${w.innerHeight}px;
    --main-width: ${m.clientWidth}px;
    --main-height: ${m.clientHeight}px;
    --header-height: ${h.clientHeight}px;
    --footer-height: ${f.clientHeight}px;
}
`;
        set_body_class(w.innerWidth);
    }
    function handlResize(ev) {
        applyDynamicStyleRules();
    }
    function setupWatchers() {
        styles.setAttribute('type', 'text/css');
        document.head.appendChild(styles);
        window.addEventListener('resize', handlResize);
        applyDynamicStyleRules();
    }
    getEl('#app', 30000).then(el => {
        setupWatchers();
        setTimeout(() => {
            el.className = 'page transition in';
            setTimeout(() => {
                el.className = '';
            }, 300);
        }, 200);
    });
})();

