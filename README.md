# Nice Numbers for Graph Labels

Implementation of Nice Numbers for Graph Labels by Paul S. Heckbert from Graphics Gems, Academic Press, 1990.

```rust
assert_eq!(
    &[100., 200., 300., 400., 500., 600.],
    LooseLabel::new(105., 543., 5).unwrap().into_iter().collect::<Vec<_>>().as_slice()
);
```
