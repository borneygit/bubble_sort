# 使用Rust实现冒泡排序

工程实现了**array_bubble_sort**方法实现冒泡排序，方法签名如下
```
fn array_bubble_sort<T: PartialOrd>(vec: &mut Vec<T>)
```

实现了两个测试用例测试i32和自定义结构体Person(实现PartialOrd trait)的排序是否正确,运行结果如下:
![测试结果](./res.png)