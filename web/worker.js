import * as rustWorker from './pkg';

async function init() {
    rustWorker.worker_set_global_scope(self);

    self.onmessage = ({ data }) => {
        try {
            console.log("worker message: " + data);
            rustWorker.worker_on_message(data);
        } catch (err) {
            console.error(err);
        }
    };
    self.postMessage('initialized');
}

init();
