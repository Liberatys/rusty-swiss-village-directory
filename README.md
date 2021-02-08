# Swiss Village Directory - Rusty

THIS IS VERY MUCH WORK IN PROGRESS!!!

## What is this??

This is a test repository to test the speed of rust when using it with ruby.
This is a trial run to later replace performance critical code with rust.

## The original Gem

[Swiss Village Directory](https://github.com/renuo/swiss-village-directory)

Swiss Village Directory is a gem to wrap the csv with villages in switerzerland.
The csv is loaded into memory and then operations can be performed on the loaded values.



## Benchmark

For now the test case is to perform 1000 searches by name. This will most often only return one value... but that
should not be a problem.

Tests are performed using rake test tasks.

The average and total runtime are taken with [stop_watch](https://github.com/danielpclark/stop_watch)


Language | Average(in s)| Total(in s) | Input size
------ | ------|----------|----
Rust | 0.002546021 | 2.546021 | 4000 |
Ruby | 0.0023     |  2.34 | 4000 |
Rust | 0.0153 | 11.5 | 14600 |
Ruby | 0.0082  |  8.2018 | 14600 |

### Slow Rust 

At the moment the rust version is a lot slower than the ruby version... I do not know what to blame for the
added runtime.

## License

**MIT** 


@ Nick Flueckiger - Liberatys - 2021
