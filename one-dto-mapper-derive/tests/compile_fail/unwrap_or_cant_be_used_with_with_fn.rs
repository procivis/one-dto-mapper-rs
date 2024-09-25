use one_dto_mapper_derive::From;

struct OptionalDto {
    age: Option<u16>,
}

#[derive(From)]
#[from(NumericDto)]
struct StringDto {
    #[from(unwrap_or = "16")]
    #[from(with_fn = "std::convert::identity")]
    age: u16,
}

fn main() {}
