use dto_mapper::From;

struct OptionalDto {
    age: Option<u16>,
}

#[derive(From)]
#[convert(from = NumericDto, into = NumericDto)]
struct StringDto {
    #[convert(unwrap_or = "16")]
    age: u16,
}

fn main() {}
