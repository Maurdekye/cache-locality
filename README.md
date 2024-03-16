A utility to test the practical effects of cache locality on the performance of random memory accesses.

At the start of the test, 1gb of random bytes are allocated to a `Vec` in the heap. Next, starting at a max step size of 1 and doubling each time, an index of a location in the vec is randomly walked up and down by random step sizes, and the position it lands on is read and tallied in a wrapping u8 sum. 

At smaller maximum step sizes, the cpu cache should allow for fast access of memory, as consecutive reads will be close to one another. Once the step sizes become larger, cache hits should become less frequent, and performance should drop.

To run the test, run `cargo run --release -- test`. Add the `--out` argument to save the test results to a csv file, which you can render a plot of with `cargo run -- plot <csv file name>`.