const wasm = require('wasm');
const algo = require('./algo');
const benchmark = require('./benchmark').benchmark;

export function main() {

    benchmark('wasm binary tree', function() {
        wasm.run_binary_tree(24);
    });

    benchmark('js binary tree', function() {
        algo.runBinaryTree(24);
    });

    benchmark('wasm nqueen', function() {
        wasm.run_nqueen(14);
    });

    benchmark('js nqueen', function() {
        algo.runNQueen(14);
    });

    benchmark('wasm fibonacci', function() {
        wasm.run_fibonacci(34);
    });

    benchmark('js fibonacci', function() {
        algo.runFibonacci(34);
    });

    // benchmark('wasm regex', function() {
    //     wasm.run_regex();
    // });

    // benchmark('js regex', function() {
    //     algo.runRegex();
    // });

    // benchmark('wasm fannkuch', function() {
    //     wasm.run_fannkuch(11);
    // });

    // benchmark('js fannkuch', function() {
    //     algo.runFannkuch(11);
    // });
}