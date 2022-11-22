import * as rust from './pkg';

async function init() {
    self.onmessage = ({ data }) => {
        let result;
        try {
            console.log("worker message: " + data);
            result = rust.worker_inc(data);
        } catch (err) {
            console.error(err);
            result = 'error';
        }
        console.log("worker result: " + result);
        self.postMessage(result);
    };
    self.postMessage('initialized');
}

init();
