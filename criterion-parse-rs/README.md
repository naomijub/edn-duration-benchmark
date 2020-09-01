# Parse with Criterion 


## Benchmarking parse: Collecting 100 samples in estimated 5.1256 s (172k iteration)
- chars-opt
```                                                                      
parse                   time:   [13.563 us 13.648 us 13.749 us]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low mild
  5 (5.00%) high severe
```

```
navigate  time:   [4.32 ns 5.04 ns 5.98 ns]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
```

```
deser  time:   [19.912 us 19.944 us 19.979 us]
(no outliers)
```