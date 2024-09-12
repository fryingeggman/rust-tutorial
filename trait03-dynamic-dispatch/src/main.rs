trait MyTrait {
    fn foo(&self) -> i32;
}

struct S1;
struct S2;
struct S3;

impl MyTrait for S1 {
    fn foo(&self) -> i32 {
        1
    }
}

impl MyTrait for S2 {
    fn foo(&self) -> i32 {
        2
    }
}

impl MyTrait for S3 {
    fn foo(&self) -> i32 {
        3
    }
}

#[allow(unused)]
struct MyStruct {
    // 这种写法是错误的，应为编译器不知道my_filed的大小
    // my_field: dyn MyTrait,

    // Box在编译期大小是确定的
    my_field: Box<dyn MyTrait>,
}

fn foo(s: Box<dyn MyTrait>) {
    println!("{}", s.foo());
}

fn foo2<T: MyTrait>(t: Box<T>) {
    println!("{}", t.foo());
}

fn main() {
    let s1 = S1 {};
    let s2 = S2 {};
    let s3 = S3 {};
    let v: Vec<&dyn MyTrait> = vec![&s1, &s2, &s3];
    for s in v {
        println!("{}", s.foo());
    }

    foo(Box::new(S1));
    foo(Box::new(S2));
    foo(Box::new(S3));

    foo2(Box::new(S1));
    foo2(Box::new(S2));
    foo2(Box::new(S3));
}
