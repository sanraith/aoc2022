import './style.css';

function onError(err) {
    const canvas = document.getElementById('canvas');
    canvas.style.display = 'none';

    const placeholder = document.getElementById('canvas-placeholder');
    const header = placeholder.getElementsByClassName('title')[0];
    header.innerHTML = 'Error';

    const error = document.getElementsByClassName('error')[0];
    error.innerHTML = 'Unhandled error in the WASM backend!<br/><em>&gt; ' + err +
        '</em><br/><br/>Check dev console for more info.<br/><br/>';
}

/**
 * Resizes the main canvas to fit the current window size,
 * and notifies the rust code about the changes.
 * @param {typeof import('./pkg')} rust
 */
function onResize(rust) {
    const cellSize = 16;
    const targetWidth = 90 * cellSize;
    const targetHeight = 50 * cellSize;
    const targetRatio = targetWidth / targetHeight;

    const vw = (document.documentElement.clientWidth ?? window.innerWidth ?? 0) - 20;
    const vh = (document.documentElement.clientHeight ?? window.innerHeight ?? 0) - 20;
    let height = targetHeight;
    let width = targetWidth;
    if (targetWidth <= vw && targetHeight <= vh) { // has enough space
    } else if (vw / vh >= targetRatio) { // viewport wider than expected
        height = Math.min(vh, targetHeight);
        width = height * targetRatio;
    } else { // viewport narrower than expected
        width = Math.min(vw, targetWidth);
        height = width / targetRatio;
    }

    let r = document.querySelector(':root');
    r.style.setProperty('--term-width', width + 'px');
    r.style.setProperty('--term-height', height + 'px');

    rust.set_scale(width / targetWidth);
    console.log(width / targetWidth);
}

const ongoing_touches = new Set();

/** 
 * @param {typeof import('./pkg')} rust
 * @param {TouchEvent} event 
 * */
function handleTouch(rust, event) {
    if (!event.changedTouches || event.changedTouches.length === 0) {
        return;
    }

    if (event.type == 'touchstart') {
        for (let t of event.changedTouches) {
            ongoing_touches.add(t.identifier);
        }
    } else if (event.type == 'touchend' || event.type == 'touchcancel') {
        for (let t of event.changedTouches) {
            ongoing_touches.delete(t.identifier);
        }
    }

    const rect = document.getElementById('canvas').getBoundingClientRect();
    const touch = event.changedTouches[0];
    let x = touch.pageX - rect.left;
    let y = touch.pageY - rect.top;

    rust.push_touch_event(x, y, event.type);
    if (ongoing_touches.size < 2 && event.type === 'touchmove') {
        event.preventDefault();
    }
}

/**
 * Registers handlers for key events to forward to the rust code.
 * @param {typeof import('./pkg')} rust
 */
function registerKeyHandlers(rust) {
    document.addEventListener('keypress', event => {
        var key = event.key;
        rust.push_key_event(key);
        event.preventDefault();
    }, false);

    document.addEventListener('keydown', (event) => {
        var key = event.key;
        if (key == 'Backspace') {
            rust.push_key_event(key);
            event.preventDefault();
        }
    }, false);

    const canvas = document.getElementById('canvas');
    canvas.addEventListener('touchstart', event => handleTouch(rust, event));
    canvas.addEventListener('touchend', event => handleTouch(rust, event));
    canvas.addEventListener('touchmove', event => handleTouch(rust, event));
    canvas.addEventListener('touchcancel', event => handleTouch(rust, event));
}

async function initWorker() {
    const worker = new Worker(new URL('./worker.js', import.meta.url));
    await new Promise((resolve, reject) => {
        worker.onmessage = ({ data }) => {
            worker.onmessage = null;
            if (data === 'initialized') {
                resolve();
            } else {
                reject();
            }
        };
    });

    worker.onmessage = ({ data }) => {
        console.log(data);
    };

    return worker;
}

async function start() {
    try {
        /** @type {Promise<typeof import('./pkg')>} */
        let rustPromise = require('./pkg');
        let rustMain = await rustPromise;
        rustMain.main_wasm();

        window.addEventListener('resize', () => onResize(rustMain));
        onResize(rustMain);
        registerKeyHandlers(rustMain);

        let worker = await initWorker();
        rustMain.set_worker(worker);
        worker.onmessage = ({ data }) => {
            console.log("main received: ", data);
            rustMain.on_worker_message(data);
        };
    } catch (err) {
        onError(err);
    }
}

start();
