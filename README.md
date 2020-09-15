# edn-duration-benchmark
Simple time-elapsed benchmarks to compare EDN-RS and Clojure native EDN performance.

## Parse with Rust using Criterion

```
Benchmarking parse: Collecting 100 samples in estimated 5.1256 s (172k iteration)  
parse                   time:   [12.143 us 12.224 us 12.316 us]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low mild
  5 (5.00%) high severe
```

## Parsing with Clojure using Criterium

```
Evaluation count : 2765940 in 60 samples of 46099 calls.
             Execution time mean : 23.230131 µs
    Execution time std-deviation : 1.666128 µs
   Execution time lower quantile : 21.708660 µs ( 2.5%)
   Execution time upper quantile : 27.339060 µs (97.5%)
                   Overhead used : 8.010259 ns
```
[@serioga](https://github.com/serioga) did some more [benchmarks here](https://gist.github.com/serioga/60499dab8248a58899d10a2687d5b77c#file-edn_benchmark-clj-L84-L134)

## Other Rust Benchmarks

```                                                                      
parse     1. time:   [12.143 us 12.224 us 12.316 us]
          2. time:   [12.499 us 12.666 us 12.843 us] 
          3. time:   [12.469 us 12.712 us 12.951 us] 
```

```
navigate  1. time:   [4.32 ns 5.04 ns 5.98 ns]
          2. time:   [11.26 ns 11.76 ns 11.38 ns]
          3. time:   [8.83 ns 8.90 ns 8.18 ns] 
```

```
deser     1. time:   [16.053 us 16.269 us 16.552 us]
          2. time:   [15.863 us 15.899 us 15.936 us]  
          3. time:   [16.118 us 16.233 us 16.372 us]
```

## Parsing edn-string into Edn:

* Rust release mode: `77.57µs`
* Rust debug mode: `266.479µs`
* Clojure `edn/read-string`: `4.712235 milis`

## Navigating in Edn value. `(-> edn :associates (get 0) :role)` `edn[":associates"][0][":role"]`:

* Rust release mode: `4.224µs`
* Rust debug mode: `22.861µs`
* Clojure `edn/read-string`: `26.333 µs`

## Deserializing Edn into struct `Entity`:
* Rust release mode: `98.359µs`
* Rust debug mode: `328.054µs`
* Clojure `edn/read-string`: `4.712235 milis`

### Test:

**String**
```
"{
    :type :human
    :first-name \"bench\"
    :last-name \"mark\"
    :age 13
    :version 0.13
    :associates [
        {
            :name :julia
            :role :adm
        }
        {
            :name :otavio
            :role :contributor
        }
        {
            :name :juxt
            :role :great-ideas
        }
    ]
}"
```

**Navigation `(-> edn :associates (get 0) :role)` and `edn[":associates"][0][":role"]`**

**Rust Entity**
```rust
#[derive(Deserialize, Debug, Clone)]
struct Entity {
    entity_type: String,
    first_name: String,
    last_name: String,
    age: usize,
    version: f64,
    associates: Vec<Person>
}

#[derive(Deserialize, Debug, Clone)]
struct Person {
    name: String,
    role: String
}
```

