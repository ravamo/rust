use my_macro::MyMacro;

#[derive(MyMacro)]
struct MyStruct;

fn main() {
    println!("{}", MyStruct::hello());
}
