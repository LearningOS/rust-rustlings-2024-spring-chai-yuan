// tests9.rs
// Rust非常擅长与C/C++和其他静态编译语言共享FFI接口，甚至可以在代码本身内部进行链接！它通过extern块实现这一点，就像下面的代码一样。

// 在extern关键字后面的短字符串表示外部导入函数所遵循的ABI。在这个练习中，使用的是"Rust"，而其他变体如标准C ABI的"C"，Windows ABI的"stdcall"也存在。

// 外部导入函数在extern块中声明，用分号标记签名的结束，而不是用花括号。一些属性可以应用于这些函数声明，以修改链接行为，例如#[link_name = ".."]用于修改实际的符号名称。

// 如果要将符号导出到链接环境中，extern关键字也可以标记在具有相同ABI字符串注释的函数定义之前。Rust函数的默认ABI实际上是"Rust"，因此如果要与纯Rust函数进行链接，可以省略整个extern术语。

// Rust默认会对符号进行名称修饰，就像C++一样。为了取消这种行为并使这些函数可通过名称寻址，可以应用属性#[no_mangle]。

// 在这个练习中，你的任务是让测试用例能够调用Foo模块中的my_demo_function函数。my_demo_function_alias是my_demo_function的别名，所以测试用例中的这两行代码应该调用相同的函数。

// 除了添加两行属性之外，你不应该修改任何现有的代码。
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.

extern "Rust" {
    #[link_name = "my_demo_function"]
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
