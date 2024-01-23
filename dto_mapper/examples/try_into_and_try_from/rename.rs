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

#[derive(TryFrom, TryInto)]
#[try_from(T = PersonDto, Error = String)]
#[try_into(T = PersonDto, Error = String)]
struct Person {
    #[try_from(rename = "name")]
    #[try_into(rename = "name")]
    #[try_into(infallible)]
    full_name: UserName,
}

fn main() {
    let p = Person {
        full_name: UserName("Joe".to_string()),
    };

    let p2 = PersonDto::try_from(p).unwrap();
    let _p3 = Person::try_from(p2).unwrap();
}
