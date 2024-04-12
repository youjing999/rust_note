### 1.常量
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

### 2.隐藏
```rust
let spaces = 1;
let spaces = spaces.len();

print!("{}",spaces);
```

### 3.数值类型
```rust
//数值类型
let x = 2.0;
let y:f32 = 3.0;
let sum = 1 + 2;
let dif  = 6.5 - 1.2;
let pro = 4 + 30;
let qut = 56.7 / 32.2;
let reminder = 54 / 4;
let x = 'z';
let y:char = '$';
let z = "😁";
```

### 4.复合类型
#### 4.1元组
```rust
元组，可以将多个类型的多个值放在一个类型里面。长度是固定的，一旦声明就无法改变
let tup = (500, 6.4, 1);
println!("{}, {}, {}", tup.0, tup.1, tup.2);
使用模式匹配来结构元组
let (x, y, z) = tup;
println!("{} | {} | {}", x, y, z)
```
#### 4.2数组
```rust
数组，可以将多个值放在一个类型里，但是每个元素的类型必须相同，长度固定
let list: [i32; 6] = [1,2,3,4,5,6];
let a: [i32; 5] = [3;5];  // 相当于 let a = [3,3,3,3,3];
println!("{}", a[2]);
```
### 5.函数
```rust
let y = {
    let x = 1;
    x + 3  // 加上; 这行代码就变成一个语句，而语句是没有返回值的，相当于()
};
println!("The value of y is : {}", y)
```
具有表达式的函数
```rust
fn five() -> i32 {
    5
}
let x = five();
println!("The value of x is: {x}");
```
### 6.if
```rust
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

else if
```rust
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
```
在 let 语句中使用 if
```rust
let condition = true;
let number = if condition { 5 } else { 6 };

println!("The value of number is: {number}");
```

### 7.循环
rust提供了3种循环，loop, while, for
#### 7.1 loop, 告诉Rust反复的执行一块代码，直到触发停止条件，所以在循环里可以用break关键字来告诉程序合适停止循环
```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
println!("最终的数字结果为: {}", result)
```
#### 7.2 while 条件循环，每次执行循环体之前都判断一次条件
```rust
let mut number = 3;
while number != 0 {
    println!("number 数值: {}", number);
    number = number - 1;
}
println!("循环结束！")
```
#### 7.3 for循环，可以针对集合中的每个元素执行一些代码
```rust
let alist = [10, 20, 30, 40, 50, 60];
for e in alist.iter() {
    println!("值为: {}", e)
}
```

range 循环，标准库，制定一个开始数字和一个结束数字，range可以生成之间的数字（不包含结束），rev方法可以反转range
```rust
for n in (1..4).rev() {  // 用了rev是倒序，3.2.1
    println!("数字: {}", n);
}
```

### 8. 所有权
rust的核心特性就是所有权
所有程序运行时都必须管理它们使用计算机内存的方式
有些语言有垃圾收集机制（Java、C#、Python），在程序运行时，它们会不断地寻找不再使用的内存，但是会带来一些性能开销
在其他语言中，程序员必须显式地分配和释放内存
rust采用了第三种方式
内存是通过一个所有权系统来管理，其中包含一组编译器在编译时检查的规则
当程序运行时，所有权特性不会减慢程序的运行速度 

栈内存（stack）和 堆内存（heap）
在rust中，一个值是在stack上还是在heap上对语言的行为和你为什么要做某些决定是有更大影响的
stack，按照值的接收顺序来储存，按相反的顺序将它们移除（后进先出，LIFO）
-添加数据叫 压入栈
-移除数据叫 弹出栈
所有存储在stack上的数据必须拥有已知的固定的大小，编译时大小未知的数据或运行时大小可能发生变化的数据必须存放在heap上
heap，内存组织性差一点，当把数据放入heap时，会请求一定数量的空间
操作系统在heap里找到一块足够大的空间，把它标记为在用，并返回一个指针，也就是这个空间的地址，这个过程叫做在heap上进行分配，有时仅仅称为”分配“
把值压入stack上不叫分配，并且因为指针是已知的固定大小的，可以把指针存放在stack上
如果想要实际数据，必须使用指针来定位
在heap上分配空间需要做更多的工作，操作系统首先需要找一个足够大的空间来存放数据，然后做好记录方便下次分配
访问heap中的数据要比访问stack中的数据慢，因为需要通过指针才能找到heap中的数据，对于现代的处理器来说，由于缓存的缘故，如果指令在内存中跳转的次数越少，那么速度就越快
如果数据存放的距离比较近，那么处理器的处理速度就会更快一些（stack上）
如果数据之间的距离比较远，那么处理速度就会慢一些（heap上）在heap上分配大量的空间也是需要时间的

所有权解决的问题：
- 跟踪代码的那些部分正在使用heap的那些数据
- 最小化heap上的重复数据量
- 清理heap上未使用的数据以避免空间不足

所以，管理heap数据就是所有权存在的原因