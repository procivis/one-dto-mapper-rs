use one_dto_mapper::{TryFrom, TryInto};

struct UserName(String);

impl TryFrom<String> for UserName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(UserName(value))
    }
}

struct OptionalDto {
    name: String,
    age: Option<u16>,
}

#[derive(TryFrom)]
#[try_from(T = OptionalDto, Error = String)]
struct FromOptionalDto {
    name: UserName,

    #[try_from(unwrap_or = "16")]
    age: u16,
}

struct NonOptionalDto {
    name: UserName,
    age: u16,
}

#[derive(TryInto)]
#[try_into(T = NonOptionalDto, Error = String)]
struct IntoNonOptionalDto {
    name: String,

    #[try_into(unwrap_or = "21")]
    age: Option<u16>,
}

enum SomeEnumDto {
    Val { age: u16 },
    OtherValue(u16),
}

#[derive(TryInto)]
#[try_into(T = SomeEnumDto, Error = String)]
enum IntoSomeEnumDto {
    Val {
        #[try_into(unwrap_or = "16")]
        age: Option<u16>,
    },
    OtherValue(#[try_into(unwrap_or = "16")] Option<u16>),
}

fn main() {
    let p = OptionalDto {
        name: Default::default(),
        age: None,
    };
    let _p2 = FromOptionalDto::try_from(p).unwrap();
}
