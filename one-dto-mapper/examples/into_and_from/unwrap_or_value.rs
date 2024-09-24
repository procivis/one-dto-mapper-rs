use one_dto_mapper::{From, Into};

struct OptionalDto {
    age: Option<u16>,
}

#[derive(From)]
#[from(OptionalDto)]
struct FromOptionalDto {
    #[from(unwrap_or = "16")]
    age: u16,
}

struct NonOptionalDto {
    age: u16,
}

#[derive(Into)]
#[into(NonOptionalDto)]
struct IntoNonOptionalDto {
    #[into(unwrap_or = "21")]
    age: Option<u16>,
}

enum SomeEnumDto {
    Val { age: u16 },
    OtherValue(u16),
}

#[derive(Into)]
#[into(SomeEnumDto)]
enum IntoSomeEnumDto {
    Val {
        #[into(unwrap_or = "16")]
        age: Option<u16>,
    },
    OtherValue(#[into(unwrap_or = "16")] Option<u16>),
}

#[derive(Into, From)]
#[from(OptionalDto)]
#[into(NonOptionalDto)]
struct FromIntoOptionalDto {
    #[from(unwrap_or = "16")]
    age: u16,
}

fn main() {
    let p = OptionalDto { age: None };
    let _p2 = FromOptionalDto::from(p);
}
