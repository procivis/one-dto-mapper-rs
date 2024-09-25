use one_dto_mapper_derive::From;

enum X {
    Variant1(u32, f32),
}

#[derive(From)]
#[from(Y)]
enum Y {
    Variant1(u32, f32),
}

fn main() {}
