# edn-duration-benchmark
Simple time-elapsed benchmarks to compare EDN-RS and Clojure native EDN performance.

## Parsing edn-string into Edn:

* Rust release mode: `110.932µs`
* Rust debug mode: `487.179µs`
* Clojure `edn/parse-string`: `4.712235 milis`

## Navigating in Edn value. 3 blocks:

* Rust release mode: `6.972µs`
* Rust debug mode: `24.294µs`
* Clojure `edn/parse-string`: `39.914 µs`


## Deserializing Edn into struct `Entity`:
* Rust release mode: `142.983µs`
* Rust debug mode: `556.568µs`
* Clojure `edn/parse-string`: `4.712235 milis`

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

