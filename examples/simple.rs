use dto_mapper::From;

struct PersonDto {
    name: String,
    age: u16,
    gender: GenderDto,
}
enum GenderDto {
    M,
    F
}


#[derive(From)]
#[convert(into = PersonDto)]
struct Person {
    name: String,
    age: u16,
    gender: Gender,
}

#[derive(From)]
#[convert(into = GenderDto)]
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

    let _p2 = PersonDto::from(p);
}