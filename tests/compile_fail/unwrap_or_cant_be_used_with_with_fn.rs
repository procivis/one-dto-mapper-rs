use dto_mapper::From;

struct OptionalDto {
    age: Option<u16>,
}

#[derive(From)]
#[convert(from = NumericDto)]
struct StringDto {
    #[convert(unwrap_or = "16")]
    #[convert(with_fn = "std::convert::identity")]
    age: u16,
}

fn main() {}
