This iterator adapter holds an internal state and emit this state on each iteration.

This internal state can be [cloned](Clone).

[`scan_clone()`](Self::scan_clone) takes 2 arguments:
* An initial value which seeds the internal state.
* A closure that:
  - Takes 2 arguments: Copy of the internal state from the previous iteration and the current item.
  - Returns the new state for the next iteration.

**Example:** Basic usage

```rust
use iter_scan::IterScan;
let input = ['a', 'b', 'c', 'd', 'e', 'f'];
let output: Vec<_> = input
    .into_iter()
    .scan_clone(String::new(), |acc, x| format!("{acc}{x}"))
    .collect();
assert_eq!(output, ["a", "ab", "abc", "abcd", "abcde", "abcdef"]);
```
