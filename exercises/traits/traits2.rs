// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String>{
    fn append_bar(mut self) -> Self{
        self.push(String::from("Bar"));
        self
    }
}
//为什么这样写？
//impl AppendBar for Vec<String>：我们是为 Vec<String>（字符串向量）实现这个 trait，而不是单独的 String。
//mut self：因为我们需要往 self 里面 push 新的元素，所以必须获取它的所有权并声明为可变的（mut）。
//self.push(...)：向数组末尾添加一个元素。
//self（最后一行没有分号）：把修改好的数组作为返回值交出去，满足 -> Self 的要求。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
