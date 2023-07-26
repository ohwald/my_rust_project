use std::ops::Add;

// 定义 MyNumber 类型
#[derive(Debug, Clone, Copy)]
struct MyNumber(i32);

// 为 MyNumber 实现 Add trait，支持加法运算
impl Add for MyNumber {
    type Output = MyNumber;

    fn add(self, other: MyNumber) -> MyNumber {
        MyNumber(self.0 + other.0)
    }
}

// 在 add_numbers 函数中使用 Clone trait 来克隆数据
fn add_numbers<Rhs: Add<Output = MyNumber> + Clone>(a: &Rhs, b: &Rhs) -> MyNumber {
    a.clone() + b.clone()
}

pub(crate) fn invoke_add_numbers() {
    let num1 = MyNumber(10);
    let num2 = MyNumber(20);
    let num3 = MyNumber(30);

    // 调用 add_numbers 函数，传入实现了 Add trait 的对象
    let result = add_numbers(&num1, &num2);
    println!("{:?} + {:?} = {:?}", num1, num2, result);

    let result2 = add_numbers(&num2, &num3);
    println!("{:?} + {:?} = {:?}", num2, num3, result2);
}
