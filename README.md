# rust-enum-vs-trait-benchmark
Simple benchmark to compare the performance of Rust enums and traits


## Approx benchmark results

Run on a Core i7-3770 @ 3.4GHz, Windows 10

    running 4 tests
    test test_enum_variant ... ignored
    test test_trait_variant ... ignored
    test bench_enum_variant  ... bench:      75,967 ns/iter (+/- 963)
    test bench_trait_variant ... bench:     177,434 ns/iter (+/- 3,555)
