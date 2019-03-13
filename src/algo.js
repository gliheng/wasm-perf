const TreeNode = function(left, right) {
    this.left  = left;
    this.right = right;
};


function bottomUpTree(depth){
    return depth>0 ? new TreeNode(
            bottomUpTree(depth-1),
            bottomUpTree(depth-1)
    ) : null;
};

exports.runBinaryTree = function(depth) {
    bottomUpTree(depth);
};

// require.extensions['.txt'] = function (module, filename) {
//     var fs = require('fs');
//     module.exports = fs.readFileSync(filename, 'utf8');
// };

var i = require('./regexredux-input.txt');
exports.runRegex = function() {
    var ilen = i.length, clen, j,
    q = [/agggtaaa|tttaccct/ig, /[cgt]gggtaaa|tttaccc[acg]/ig,
      /a[act]ggtaaa|tttacc[agt]t/ig, /ag[act]gtaaa|tttac[agt]ct/ig,
      /agg[act]taaa|ttta[agt]cct/ig, /aggg[acg]aaa|ttt[cgt]ccct/ig,
      /agggt[cgt]aa|tt[acg]accct/ig, /agggta[cgt]a|t[acg]taccct/ig,
      /agggtaa[cgt]|[acg]ttaccct/ig];
  
  i = i.replace(/^>.*\n|\n/mg, '');
  clen = i.length;
  for(j = 0; j<q.length; ++j) {
    var qj = q[j], m = i.match(qj);
    console.log(qj.source, m ? m.length : 0);
  }
  
  i = i.replace(/tHa[Nt]/g, "<4>")
       .replace(/aND|caN|Ha[DS]|WaS/g, "<3>")
       .replace(/a[NSt]|BY/g, "<2>")
       .replace(/<[^>]*>/g, "|")
       .replace(/\|[^|][^|]*\|/g, "-");
  
  console.log(["", ilen, clen, i.length].join("\n"));
};


function fannkuch(n) {
    var p = [], q = [], s = [];
    var sign = 1, maxflips = 0, sum = 0, m = n-1;
    for(var i=0; i<n; i++){ p.push(i); q.push(i); s.push(i); }
    do {
       // Copy and flip.
       var q0 = p[0];                                     // Cache 0th element.
       if (q0 != 0){
          for(var i=1; i<n; i++) q[i] = p[i];             // Work on a copy.
          var flips = 1;
          do { 
             var qq = q[q0]; 
             if (qq === 0){                               // ... until 0th element is 0.
                sum += sign*flips;
            if (flips > maxflips) maxflips = flips;   // New maximum?
                break; 
             } 
          q[q0] = q0; 
         if (q0 >= 3){
            var i = 1, j = q0 - 1, t;
                do { t = q[i]; q[i] = q[j]; q[j] = t; i++; j--; } while (i < j); 
             }
         q0 = qq; flips++;
          } while (true); 
       }
       // Permute.
       if (sign === 1){
          var t = p[1]; p[1] = p[0]; p[0] = t; sign = -1; // Rotate 0<-1.
       } else { 
          var t = p[1]; p[1] = p[2]; p[2] = t; sign = 1;  // Rotate 0<-1 and 0<-1<-2.
          for(var i=2; i<n; i++){ 
         var sx = s[i];
         if (sx != 0){ s[i] = sx-1; break; }
         if (i === m) return [sum,maxflips];      // Out of permutations.
         s[i] = i;
         // Rotate 0<-...<-i+1.
         t = p[0]; for(var j=0; j<=i; j++){ p[j] = p[j+1]; } p[i+1] = t;
          }
       }
    } while (true);
 }
 
 exports.runFannkuch = function(n) {
    var pf = fannkuch(n);
    // console.log(pf[0] + "\n" + "Pfannkuchen(" + n + ") = " + pf[1]);   
 }
 

class NQueen {
    constructor(n) {
        this.n = n;
        this.ret = 0;
    }
    solve() {
        let c = new Array(this.n);
        c.fill(false);
        let d1 = new Array(2 * this.n - 1);
        d1.fill(false);
        let d2 = new Array(2 * this.n - 1);
        d2.fill(false);

        this.helper(0, c, d1, d2);
        return this.ret;
    }

    helper(i, c, d1, d2) {
        if (i == this.n) {
            this.ret += 1;
            return;
        }
        for (let j = 0; j < this.n; j++) {
            let idx1 = i + j;
            let idx2 = i + this.n - j - 1;
 
            if (!c[j] && !d1[idx1] && !d2[idx2]) {
                c[j] = true;
                d1[idx1] = true;
                d2[idx2] = true;
                this.helper(i + 1, c, d1, d2);
                c[j] = false;
                d1[idx1] = false;
                d2[idx2] = false;
            }
        }
    }
}

exports.runNQueen = function(n) {
    let q = new NQueen(n);
    return q.solve();
}

function fibonacci(n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}

exports.runFibonacci = function(n) {
    return fibonacci(n);
}