use one_dto_mapper_derive::From;

struct Tuple2(u32, String);

#[derive(From)]
#[from(Tuple2)]
struct Tuple(u32, String);

fn main() {}
