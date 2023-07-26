// 定义一个 Trait，为每种类型定义方法
struct Type1;
struct Type2;
struct Type3;

trait MyTrait {
    fn my_method(&self);
}

// 实现 Trait for Type1
impl MyTrait for Type1 {
    fn my_method(&self) {
        println!("This is Type1's method.");
    }
}

// 实现 Trait for Type2
impl MyTrait for Type2 {
    fn my_method(&self) {
        println!("This is Type2's method.");
    }
}

// 实现 Trait for Type3
impl MyTrait for Type3 {
    fn my_method(&self) {
        println!("This is Type3's method.");
    }
}

pub(crate) fn method2() {
    // 使用 Trait Object 包裹三种不同类型
    let objects: Vec<Box<dyn MyTrait>> = vec![Box::new(Type1), Box::new(Type2), Box::new(Type3)];

    // 遍历 Vec 并调用各自的方法
    for object in &objects {
        object.my_method();
    }
}
