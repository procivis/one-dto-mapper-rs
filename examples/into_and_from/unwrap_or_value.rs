use dto_mapper::From;

struct OptionalDto {
    age: Option<u16>,
}

#[derive(From)]
#[convert(from = OptionalDto)]
struct FromOptionalDto {
    #[convert(unwrap_or = "16")]
    age: u16,
}

struct NonOptionalDto {
    age: u16,
}

#[derive(From)]
#[convert(into = NonOptionalDto)]
struct IntoNonOptionalDto {
    #[convert(unwrap_or = "21")]
    age: Option<u16>,
}

enum SomeEnumDto {
    Val { age: u16 },
    OtherValue(u16),
}

#[derive(From)]
#[convert(into = SomeEnumDto)]
enum IntoSomeEnumDto {
    Val {
        #[convert(unwrap_or = "16")]
        age: Option<u16>,
    },
    OtherValue(#[convert(unwrap_or = "16")] Option<u16>),
}

#[derive(From)]
#[convert(from = OptionalDto, into = NonOptionalDto)]
struct FromIntoOptionalDto {
    #[convert(from_unwrap_or = "16")]
    age: u16,
}

fn main() {
    let p = OptionalDto { age: None };
    let _p2 = FromOptionalDto::from(p);
}
