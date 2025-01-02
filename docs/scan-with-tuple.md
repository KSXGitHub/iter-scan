This iterator adapter holds an internal state and emit this state on each iteration.

This adapter should be used when the internal state can neither be [cloned](Clone) nor [copied](Copy).

`scan_with_tuple()` takes 2 arguments:
* An initial value which seeds the internal state.
* A closure that:
  - Takes 2 arguments: Copy of the internal state from the previous iteration and the current item.
  - Returns a tuple of the new state and a value.

**Example:** Basic usage.

```rust
use iter_scan::IterScan;
enum SourceItem {
    Separator,
    Value(&'static str),
}
let source = [
    SourceItem::Value("zero"),
    SourceItem::Value("one"),
    SourceItem::Value("two"),
    SourceItem::Separator,
    SourceItem::Value("three"),
    SourceItem::Value("four"),
    SourceItem::Separator,
    SourceItem::Value("five"),
    SourceItem::Separator,
    SourceItem::Value("six"),
];
let tagged: Vec<_> = source
    .into_iter()
    .scan_with_tuple(0u32, |prev_tag, item| match item {
        SourceItem::Separator => (prev_tag + 1, None),
        SourceItem::Value(value) => (prev_tag, Some((prev_tag, value))),
    })
    .flatten()
    .collect();
assert_eq!(
    &tagged,
    &[
        (0, "zero"),
        (0, "one"),
        (0, "two"),
        (1, "three"),
        (1, "four"),
        (2, "five"),
        (3, "six"),
    ],
);
```
