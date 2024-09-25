use one_dto_mapper::Into;

#[derive(Into)]
#[into(Y)]
struct X {
    a: String,
    b: f32,
    #[into(skip)]
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

#[derive(Into)]
#[into(User)]
enum UserDto {
    Bar,
    Foo {
        name: String,
        surname: String,

        #[into(skip)]
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
