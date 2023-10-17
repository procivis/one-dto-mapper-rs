
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

fn main() {

    let x = X {
        a: "A".to_string(),
        b: 100.0,
        c: None,
    };

    let _y = Y::from(x);
}