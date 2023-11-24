Derives `From` implementation for types with similar shape.
See [examples](./examples) for how to use it.

Add to your `Cargo.toml`

```toml
dto_mapper = { git = "ssh://git@gitlab.procivis.ch:procivis/one/dto-mapper-rs.git" }
```

## Mapping `Optional<T>` into `T`

unwrap_or, into_unwrap_or and from_unwrap_or attributes can be used to map `Optional<T>` into `T` using default value.

Example:

```rust
struct OptionalDto {
    age: Option<u16>,
}

#[derive(From)]
#[convert(from = OptionalDto)]
struct FromOptionalDto {
    #[convert(unwrap_or = "16")]
    age: u16,
}
```

`age` in `FromOptionalDto` will be set to `16` if original value is `None`. More examples can be found [here](examples/unwrap_or_value.rs).

This attribute cannot be combined with `with_fn` or `with_fn_ref` attributes.
