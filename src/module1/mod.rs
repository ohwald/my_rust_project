// 定义三个不同的类型
struct Type1;
struct Type2;
struct Type3;

// 为每个类型实现各自的方法
impl Type1 {
    fn method1(&self) {
        println!("This is Type1's method1.");
    }
}

impl Type2 {
    fn method2(&self) {
        println!("This is Type2's method2.");
    }
}

impl Type3 {
    fn method3(&self) {
        println!("This is Type3's method3.");
    }
}

// 使用枚举包裹三个不同的类型
enum EnumType {
    Variant1(Type1),
    Variant2(Type2),
    Variant3(Type3),
}

pub(crate) fn method1() {
    // 创建包含不同类型的 Vec
    let types_vec: Vec<EnumType> = vec![
        EnumType::Variant1(Type1),
        EnumType::Variant2(Type2),
        EnumType::Variant3(Type3),
    ];

    // 遍历 Vec 并调用各自的方法
    for item in &types_vec {
        match item {
            EnumType::Variant1(type1) => type1.method1(),
            EnumType::Variant2(type2) => type2.method2(),
            EnumType::Variant3(type3) => type3.method3(),
        }
    }
}
