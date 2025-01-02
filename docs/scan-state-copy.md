This iterator adapter holds an internal state and emit a tuple of this state and a mapped value on each iteration.

This internal state can be [copied](Copy).

`scan_state_copy()` takes 2 arguments:
* An initial value which seeds the internal state.
* A closure that:
  - Takes 2 arguments: Copy of the internal state from the previous iteration and the current item.
  - Returns the new state and the mapped value of the item.

**Example:** Group indexing.

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
    .scan_state_copy(0u32, |count, item| match item {
        SourceItem::Separator => (count + 1, None),
        SourceItem::Value(value) => (count, Some(value)),
    })
    .flat_map(|(count, item)| item.map(|item| (count, item)))
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
