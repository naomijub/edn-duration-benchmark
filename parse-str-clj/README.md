# parse-str-clj

* Parse edn: `"Elapsed time: 4.712235 msecs"` with `clojure.core/time`
* Navigate to ":role": `"Elapsed time: 26.333 µsecs"` with `clojure.core/time`

### Using Criterium
* Just remember that criterium is a microbenchmark tool that removes GC time and has JVM optimizations, so it is far from how most people use it and GC time is something very important to consider when executing your API.

Criterium measures the computation time of an expression. It is designed to address some of the pitfalls of benchmarking, and benchmarking on the JVM in particular.
- statistical processing of multiple evaluations
- inclusion of a warm-up period, designed to allow the JIT compiler to optimise its code
- purging of gc before testing, to isolate timings from GC state prior to testing
- a final forced GC after testing to estimate impact of cleanup on the timing results

```
Evaluation count : 2765940 in 60 samples of 46099 calls.
             Execution time mean : 23.230131 µs
    Execution time std-deviation : 1.666128 µs
   Execution time lower quantile : 21.708660 µs ( 2.5%)
   Execution time upper quantile : 27.339060 µs (97.5%)
                   Overhead used : 8.010259 ns

Found 2 outliers in 60 samples (3.3333 %)
	low-severe	 2 (3.3333 %)
 Variance from outliers : 53.4686 % Variance is severely inflated by outliers
Evaluation count : 952290960 in 60 samples of 15871516 calls.
             Execution time mean : 55.693747 ns
    Execution time std-deviation : 1.590154 ns
   Execution time lower quantile : 54.464107 ns ( 2.5%)
   Execution time upper quantile : 59.918761 ns (97.5%)
                   Overhead used : 8.010259 ns

Found 7 outliers in 60 samples (11.6667 %)
	low-severe	 5 (8.3333 %)
	low-mild	 2 (3.3333 %)
```