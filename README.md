Derives `From` implementation for types with similar shape.
See [examples](./examples) for how to use it.

Add to your `Cargo.toml`

```toml
dto_mapper = { git = "ssh://git@gitlab.procivis.ch:procivis/one/dto-mapper-rs.git" }
```

## Mapping `Optional<T>` into `T`

`unwrap_or`, `into_unwrap_or` and `from_unwrap_or` attributes can be used to map `Optional<T>` into `T` using default value.

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

## Field renaming

The `rename` attribute can be used in cases where the field name is different across structures.

Example:

```rust
struct PersonDto {
    name: String,
}

#[derive(From)]
#[convert(into = PersonDto, from = PersonDto)]
struct AnotherPerson {
    #[convert(rename = "name")]
    full_name: String,
}
```

In this case `full_name` will be mapped to `name`.

More examples can be found [here](examples/rename.rs).

## Default value for field

The `replace` attribute can be used in cases where the source data type does not have any value to be used as a source for mapping. In this case, you can specify a static value that will always be used instead.

Example:

```rust
struct PersonDto {
    name: String,
}

#[derive(From)]
#[convert(from = PersonDto)]
struct FromPerson {
    name: String,

    #[convert(replace = "0u16")]
    age: u16,
}
```

In this case `age` will be allways assigned `0` when `FromPerson` is created from `PersonDto`.

More examples can be found [here](examples/replace.rs).
