![Procivis One](docs/assets/logo_dark_Procivis_One.png#gh-light-mode-only)
![Procivis One](docs/assets/logo_light_Procivis_One.png#gh-dark-mode-only)

# One DTO Mapper

Dependency of the [Procivis One Core][core], a complete solution capable of powering every element
of the digital identity credential lifecycle. See the complete solution [architecture][archi].

Derives [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) and [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) implementations for types with similar shape.
See [examples](./one-dto-mapper/examples) for how to use it.

## Infallible conversions

### Mapping `Optional<T>` into `T`

`unwrap_or` attributes can be used to map `Optional<T>` into `T` using default value.

Example:

```rust
struct OptionalDto {
    age: Option<u16>,
}

#[derive(From)]
#[from(OptionalDto)]
struct FromOptionalDto {
    #[from(unwrap_or = "16")]
    age: u16,
}
```

`age` in `FromOptionalDto` will be set to `16` if original value is `None`. More examples can be found [here](./one-dto-mapper/examples/into_and_from/unwrap_or_value.rs) and [here](./one-dto-mapper/examples/try_into_and_try_from/unwrap_or_value.rs).

This attribute cannot be combined with `with_fn` or `with_fn_ref` attributes.

### Field renaming

The `rename` attribute can be used in cases where the field name is different across structures.

Example:

```rust
struct PersonDto {
    name: String,
}

#[derive(Into, From)]
#[into(PersonDto)]
#[from(PersonDto)]
struct AnotherPerson {
    #[into(rename = "name")]
    #[from(rename = "name")]
    full_name: String,
}
```

In this case `full_name` will be mapped to `name`.

More examples can be found [here](./one-dto-mapper/examples/into_and_from/rename.rs) and [here](./one-dto-mapper/examples/try_into_and_try_from/rename.rs).

### Default value for field

The `replace` attribute can be used in cases where the source data type does not have any value to be used as a source for mapping. In this case, you can specify a static value that will always be used instead.

Example:

```rust
struct PersonDto {
    name: String,
}

#[derive(From)]
#[from(PersonDto)]
struct FromPerson {
    name: String,

    #[from(replace = "0u16")]
    age: u16,
}
```

In this case `age` will be allways assigned `0` when `FromPerson` is created from `PersonDto`.

More examples can be found [here](./one-dto-mapper/examples/into_and_from/replace.rs) and [here](./one-dto-mapper/examples/try_into_and_try_from/replace.rs).

## Failable conversions

`TryFrom` and `TryInto` macros can be used to generate failable conversions using [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) trait. They support the same feature set as `From` macro. Examples can be found [here](./one-dto-mapper/examples/try_into_and_try_from).

### Force infallible conversion

`infallible` attribute can be used nn cases when infallible conversion should be used.

Example:

```rust
struct PersonDto {
    name: String,
    age: u16,
}

#[derive(TryFrom)]
#[try_from(T = PersonDto, Error = String)]
struct Person {
    name: UserName,

    #[try_from(infallible)]
    age: u16,
}
```

In this case `age` will be converted using `From` trait instead of `TryFrom`.

## Support

Need support or have feedback? [Contact us](https://www.procivis.ch/en/contact).

## License

Some rights reserved. This library is published under the [Apache License
Version 2.0](./LICENSE).

![Procivis AG](docs/assets/logo_light_mode_Procivis.svg#gh-light-mode-only)
![Procivis AG](docs/assets/logo_dark_mode_Procivis.svg#gh-dark-mode-only)

© Procivis AG, [https://www.procivis.ch](https://www.procivis.ch).

[archi]: https://github.com/procivis#architecture
[core]: https://github.com/procivis/one-core
