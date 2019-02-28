export function main() {
    const wasm = require('wasm');
    const algo = require('./algo');

    // console.time('wasm binary tree');
    // wasm.run_binary_tree(24);
    // console.timeEnd('wasm binary tree');

    // console.time('js binary tree');
    // algo.runBinaryTree(24);
    // console.timeEnd('js binary tree');

    // console.time('wasm regex');
    // wasm.run_regex();
    // console.timeEnd('wasm regex');

    // console.time('js regex');
    // algo.runRegex();
    // console.timeEnd('js regex');

    // console.time('wasm fannkuch');
    // wasm.run_fannkuch(11);
    // console.timeEnd('wasm fannkuch');

    // console.time('js fannkuch');
    // algo.runFannkuch(11);
    // console.timeEnd('js fannkuch');

    console.time('wasm nqueen');
    wasm.run_nqueen(14);
    console.timeEnd('wasm nqueen');

    console.time('js nqueen');
    algo.runNQueen(14);
    console.timeEnd('js nqueen');

    // console.time('wasm fibonacci');
    // wasm.run_fibonacci(34);
    // console.timeEnd('wasm fibonacci');

    // console.time('js fibonacci');
    // algo.runFibonacci(34);
    // console.timeEnd('js fibonacci');
}