mod module1;
mod module2;
mod add_trait;

fn main() {
    // 方法一使用枚举将不同的类型包裹在一起，每个枚举变体都可以直接调用各自类型的方法。这样的做法在每个类型的方法具有不同参数或返回值时非常方便。但是，如果每个类型的方法具有完全不同的方法名或签名，枚举可能会变得繁琐。
    module1::method1();

    // 方法二使用了 Trait Object 来处理不同类型的实现。Trait Object 允许我们在运行时处理不同类型的对象，但在编译时我们无法确定 Trait Object 的具体类型。这使得在遍历 Vec 时调用方法更加简洁，但是可能会失去某些类型特定的功能。另外，Trait Object 会引入运行时的开销，从而增加代码的复杂性。
    module2::method2();

    // lesson4的第二个问题，通过实现Add trait，我们可以为自定义类型添加加法运算功能。然后，通过add_numbers函数中的Trait Object参数，我们可以在运行时调用不同类型的加法运算。这样的设计使得代码更加灵活，能够处理多种类型的加法操作。
    add_trait::invoke_add_numbers();
}
