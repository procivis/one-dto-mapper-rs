use one_dto_mapper_derive::From;

struct Tuple2(u32);

#[derive(From)]
#[from(Tuple2)]
struct Tuple(u32);

fn main() {}
