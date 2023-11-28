use dto_mapper::From;

struct PersonDto {
    name: String,
}

#[derive(From)]
#[convert(into = PersonDto, from = PersonDto)]
struct Person {
    #[convert(rename = "name")]
    full_name: String,
}

fn main() {
    let p = Person {
        full_name: "Joe".to_string(),
    };

    let p2 = PersonDto::from(p);
    let _p3 = Person::from(p2);
}
