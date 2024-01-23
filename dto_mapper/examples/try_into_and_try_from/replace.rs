use dto_mapper::{TryFrom, TryInto};

struct UserName(String);

impl TryFrom<String> for UserName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(UserName(value))
    }
}

impl From<UserName> for String {
    fn from(value: UserName) -> Self {
        value.0
    }
}

struct PersonDto {
    name: String,
}

#[derive(TryFrom)]
#[try_from(T = PersonDto, Error = String)]
struct FromPerson {
    name: UserName,

    #[try_from(replace = "0u16")]
    age: u16,
}

#[derive(TryFrom, TryInto)]
#[try_from(T = PersonDto, Error = String)]
#[try_into(T = PersonDto, Error = String)]
struct IntoFromPerson {
    #[try_into(infallible)]
    name: UserName,

    #[try_into(skip)]
    #[try_from(replace = "0u16")]
    age: u16,
}

fn main() {
    let p = PersonDto {
        name: "Joe".to_string(),
    };

    let p2 = IntoFromPerson::try_from(p).unwrap();
}
