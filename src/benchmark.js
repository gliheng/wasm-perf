
let running = false;
let jobs = [];

export function benchmark(name, job) {
    jobs.push([name, job]);
    run();
}

function run() {
    if (running || jobs.length == 0) return;

    running = true;
    setTimeout(() => {
        let [name, job] = jobs.shift();
        timeStart(name);
        job();
        timeEnd(name);
        running = false;
    
        run();
    }, 0);
}

let times = {};
function timeStart(name) {
    times[name] = performance.now();
}

function timeEnd(name) {
    if (!(name in times)) {
        throw "Not started";
    }
    let n = document.createElement('p');
    let t = performance.now() - times[name];
    n.textContent = `${name}: ${t}`;
    document.body.appendChild(n);
    delete times[name];
}