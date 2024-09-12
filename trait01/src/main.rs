use std::fmt::{Debug, Display};

pub trait MyTrait {
    fn some_method(&self) -> String;
    // 有默认实现，可以不用重写
    fn foo(&self) -> String {
        "Foo".to_string()
    }
    fn bar(&self) -> String {
        "Bar".to_string()
    }
}

pub trait MyTrait2 {
    fn some_method(&self) -> i32;
}

#[derive(Debug)]
struct MyStruct {
    id: i64,
    name: String,
}

// 为自己的类型实现自己的trait
impl MyTrait for MyStruct {
    fn some_method(&self) -> String {
        String::from("Hi from some_method!")
    }
    // 虽然有默认实现，还是重写了
    fn bar(&self) -> String {
        String::from("Hi from bar!")
    }
}

// 为自己的类型实现别人的trait
impl Into<String> for MyStruct {
    fn into(self) -> String {
        format!("id: {}, name: {}", self.id, self.name)
    }
}

// 为别人的类型实现自己的trait
impl MyTrait for String {
    fn some_method(&self) -> String {
        self.to_owned()
    }
}

impl MyTrait2 for MyStruct {
    fn some_method(&self) -> i32 {
        0
    }
}

#[allow(dead_code)]
// 可以需要（满足）其他的traits
trait MyError: Debug + Display {}

fn main() {
    let ms = MyStruct {
        id: 1001,
        name: "cdd".to_string(),
    };
    //如果实现了多个trait，它们中有同名方法，则需要显式调用
    println!("{}", MyTrait::some_method(&ms));
    println!("{}", MyTrait2::some_method(&ms));
    println!("{}", ms.foo());
    println!("{}", ms.bar());
}
