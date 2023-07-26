// 这个宏有三个模式，分别用于处理没有元素、一个元素和多个元素的情况。
macro_rules! my_vec {
    // 定义模式：当没有元素时，创建一个空 Vec
    () => {
        Vec::new()
    };
    // 定义模式：创建包含单个元素的 Vec
    ($elem:expr) => {
        {
            let mut temp_vec = Vec::new();
            temp_vec.push($elem);
            temp_vec
        }
    };
    // 定义模式：创建包含多个元素的 Vec
    ($($elem:expr),+) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($elem);
            )+
            temp_vec
        }
    };
}

fn main() {
    // 使用自定义宏创建不同类型的 Vec
    let empty_vec: Vec<i32> = my_vec!();
    let single_elem_vec = my_vec!(42);
    let multi_elem_vec = my_vec!(1, 2, 3, 4, 5);

    println!("Empty Vec: {:?}", empty_vec);
    println!("Single Element Vec: {:?}", single_elem_vec);
    println!("Multiple Element Vec: {:?}", multi_elem_vec);
}
