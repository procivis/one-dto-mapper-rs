use dto_mapper::From;

enum User {
    Anonymous,
    User1 { name: String, surname: String },
    User2(User2),
}

struct User2 {
    token: String,
}

#[derive(From)]
#[convert(from = "User")]
enum UserDto {
    Anonymous,
    User1 { name: String, surname: String },
    User2(User2),
}

#[derive(From)]
#[convert(from = "User2")]
struct UserWithToken {
    token: Token,
}

struct Token(String);

impl From<String> for Token {
    fn from(value: String) -> Self {
        Self(value)
    }
}

fn main() {
    let user = User::User2(User2 {
        token: "1B777A92".to_string(),
    });

    let _user_dto = UserDto::from(user);
}
