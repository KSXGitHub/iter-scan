This iterator adapter holds an internal state and emit this state on each iteration.

This internal state can be [copied](Copy).

`scan_copy()` takes 2 arguments:
* An initial value which seeds the internal state.
* A closure that:
  - Takes 2 arguments: Copy of the internal state from the previous iteration and the current item.
  - Returns the new state for the next iteration.

**Example:** Basic usage

```rust
use iter_scan::IterScan;
let input = [2, 3, 4, 5];
let output: Vec<u64> = input
    .into_iter()
    .scan_copy(1, |acc, x| acc * x)
    .collect();
assert_eq!(output, [2, 6, 24, 120]);
```
