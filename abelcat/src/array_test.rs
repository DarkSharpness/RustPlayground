use std::ops::{Index, IndexMut};

// 定义一个包含整数数组的结构体
struct MyArray {
    data: [i32; 5],
}

// 实现 Index trait 用于读取值
impl Index<usize> for MyArray {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// 实现 IndexMut trait 用于修改值
impl IndexMut<usize> for MyArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

pub fn main() {
    let mut my_array = MyArray { data: [1, 2, 3, 4, 5] };

    // 通过下标访问数组元素
    println!("Value at index 2: {}", my_array[2]);

    // 通过下标修改数组元素
    my_array[2] = 10;
    println!("Updated value at index 2: {}", my_array[2]);
}
