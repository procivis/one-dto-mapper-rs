use dto_mapper::TryInto;

struct UserName(String);

impl TryFrom<String> for UserName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(UserName(value))
    }
}

#[derive(TryInto)]
#[try_into(T = Y, Error = String)]
struct X {
    a: String,
    #[try_into(skip)]
    c: Option<String>,
}

struct Y {
    a: UserName,
}

enum User {
    Bar,
    Foo { name: UserName, surname: UserName },
}

#[derive(TryInto)]
#[try_into(T = User, Error = String)]
enum UserDto {
    Bar,
    Foo {
        name: String,
        surname: String,

        #[try_into(skip)]
        age: Option<u16>,
    },
}

fn main() {
    let x = X {
        a: "A".to_string(),
        c: None,
    };

    let _y = Y::try_from(x).unwrap();
}
