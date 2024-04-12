fn main() {
    // let spaces = 1;
    // let spaces = spaces.len();
    //
    // print!("{}",spaces);
    //
    // let guess:u32 = "40".parse().expect("not a num");
    // print!("{}",guess)

    //数值类型
    // let x = 2.0;
    // let y:f32 = 3.0;
    // let sum = 1 + 2;
    // let dif  = 6.5 - 1.2;
    // let pro = 4 + 30;
    // let qut = 56.7 / 32.2;
    // let reminder = 54 / 4;
    // let x = 'z';
    // let y:char = '$';
    // let z = "😁";


    // let y = {
    //     let x = 1;
    //     x + 3  // 加上; 这行代码就变成一个语句，而语句是没有返回值的，相当于()
    // };
    // println!("The value of y is : {}", y)

    // fn five() -> i32 {
    //     5
    // }
    // let x = five();
    // println!("The value of x is: {x}");

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("最终的数字结果为: {}", result)
    //
    // let mut number = 3;
    // while number != 0 {
    //     println!("number 数值: {}", number);
    //     number = number - 1;
    // }
    // println!("循环结束！")

    // let list = [10, 20, 30, 40, 50, 60];
    // for e in list.iter() {
    //     println!("值为: {e}")
    // }
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);



}
