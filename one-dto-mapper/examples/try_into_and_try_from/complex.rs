use one_dto_mapper::TryFrom;

struct UserName(String);

impl TryFrom<String> for UserName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

enum User {
    Anonymous,
    User1 { name: String, surname: String },
    User2(User2),
}

struct User2 {
    token: String,
}

#[derive(TryFrom)]
#[try_from(T = User, Error = String)]
enum UserDto {
    Anonymous,
    User1 { name: UserName, surname: UserName },
    User2(#[try_from(infallible)] User2),
}

#[derive(TryFrom)]
#[try_from(T = User2, Error = String)]
struct UserWithToken {
    token: Token,
}

struct Token(String);

impl TryFrom<String> for Token {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

fn main() {
    let user = User::User2(User2 {
        token: "1B777A92".to_string(),
    });

    let _user_dto = UserDto::try_from(user);
}
