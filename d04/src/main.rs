fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("print tup {} {} {}", tup.0, tup.1, tup.2);
    // 使用模式匹配获取 tuple 的值
    let (x, y, z) = tup;
    println!("print tup {} {} {}", x, y, z);
    // 定义一个数组 a
    let a: [i32; 5] = [1, 1, 1, 1, 1];
    // 定义一个数组 b 实际里面值和a是一样的
    let b: [i32; 5] = [1; 5];

    println!("a {}", a[1]);
    println!("b {}", b[1]);
}
