use dto_mapper::From;

#[derive(From)]
#[convert(into = "Y")]
struct X {
    a: String,
    b: f32,
    #[convert(skip)]
    c: Option<String>,
}

struct Y {
    a: String,
    b: f32,
}

enum User {
    Bar,
    Foo { name: String, surname: String },
}

#[derive(From)]
#[convert(into = "User")]
enum UserDto {
    Bar,
    Foo {
        name: String,
        surname: String,

        #[convert(skip)]
        age: Option<u16>,
    },
}

fn main() {
    let x = X {
        a: "A".to_string(),
        b: 100.0,
        c: None,
    };

    let _y = Y::from(x);
}
