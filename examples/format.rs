use description::Description;

const SOME_CONSTANT: usize = 5;

#[derive(Description)]
enum SomeStatusEnum {
    #[description("the constant is {SOME_CONSTANT}, and the max u32 is {}", u32::MAX)]
    ShowConstant,

    #[description("i'm not showing the constant")]
    DontShowConstant,
}

fn main() {
    let charger = SomeStatusEnum::ShowConstant;

    println!("enum message: {}", charger.description());
}