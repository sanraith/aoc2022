import './style.css';

function resize(rust) {
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

function handleKeypresses(rust) {
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
}

async function start() {
    let rust;
    try {
        rust = await require('./pkg');
        rust.main_wasm();
    } catch (err) {
        onError(err);
    }

    window.addEventListener('resize', () => resize(rust));
    resize(rust);
    handleKeypresses(rust);
}

start();
