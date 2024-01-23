use dto_mapper::{Into, TryInto};

struct UserName(String);

impl TryFrom<String> for UserName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(UserName(value))
    }
}

struct PersonDto {
    name: UserName,
    age: u16,
    gender: GenderDto,
}

enum GenderDto {
    M,
    F,
}

#[derive(TryInto)]
#[try_into(T = PersonDto, Error = String)]
struct Person {
    name: String,

    #[try_into(infallible)]
    age: u16,

    #[try_into(infallible)]
    gender: Gender,
}

#[derive(Into)]
#[into(GenderDto)]
enum Gender {
    M,
    F,
}

fn main() {
    let p = Person {
        name: "Joe".to_string(),
        age: 42,
        gender: Gender::M,
    };

    let _p2 = PersonDto::try_from(p).unwrap();
}
