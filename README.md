# edn-duration-benchmark
Simple time-elapsed benchmarks to compare EDN-RS and Clojure native EDN performance.

## Parsing edn-string into Edn:

* Rust release mode: `77.57µs`
* Rust debug mode: `266.479µs`
* Clojure `edn/read-string`: `4.712235 milis`

## Navigating in Edn value. 3 blocks:

* Rust release mode: `4.224µs`
* Rust debug mode: `22.861µs`
* Clojure `edn/read-string`: `26.333 µs`

## Deserializing Edn into struct `Entity`:
* Rust release mode: `110.358µs`
* Rust debug mode: `357.054µs`
* Clojure `edn/read-string`: `4.712235 milis`

## Parse with Rust using Criterion

```
Benchmarking parse: Collecting 100 samples in estimated 5.1256 s (172k iteration)  
parse                   time:   [13.563 us 13.648 us 13.749 us]
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

**Navegation `[:associates 0 :role]`**

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

