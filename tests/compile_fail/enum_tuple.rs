use dto_mapper::From;

enum X {
    Variant1(u32, f32),
}

#[derive(From)]
#[convert(from = "Y")]
enum Y {
    Variant1(u32, f32),
}

fn main() {}