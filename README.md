
# python_with_rust

演示 Python 如何调用一个 Rust 项目
1. Python 传给 Rust 这些数据结构：str, int, float, list, dict 等
2. Rust 返回给 Python 这些数据结构：String, i32, f64, Vec<i32>, Vec<String>
3. Python 使用 Rust 类

## 一、环境要求

1. 安装 rust 和 cargo，并升级到最新版本（略）
2. python 环境
```
pip install --upgrade pip
pip install -r requirements.txt 
```
3. cc（c编译器）
```
sudo apt update
sudo apt install build-essential
sudo apt install gcc
```

## 二、测试

安装
```shell
pip install .
```

测试
```shell
python examples/example1.py # 测试可以调用 Python 函数
python examples/example2.py # 测试可以调用 Rust 函数
python examples/example3.py # 测试相互传递 list、string 等数据
python examples/example4.py # 测试可以调用 Rust 类
```

## 三、说明

- `./my_rust_project1` 是一个纯 Rust 项目
- `./my_rust_project` 是一个中间层，它使用 pyo3，调用 `my_rust_project1` 并被 Python 调用
    - 目的是让 `./my_rust_project1` 不用考虑被 python 调用时的各种问题，成为纯粹的 Rust 项目，不被 `pyo3` 污染
- 但是在中间层 `./my_rust_project` 执行 `cargo build` 会失败（可能与什么配置有关）
- 另一个方案是 ffi 方法，使用 C 标准编译 `my_rust_project1`，编译后的代码可以被 Python(ctypes)/C/Rust/Java 调用。具体参见 [郭飞的笔记](https://www.guofei.site/2022/08/28/rust2.html#Python%20%E8%B0%83%E7%94%A8%20Rust%20%E7%BC%96%E8%AF%91%E5%90%8E)
    - 缺点是需要自定义数据类型，并有内存泄露的风险。优点是某些情况下潜在性能更高
    - 使用 `pyo3` 是以一个比较好的实践，这个项目展示使用 `pyo3` 的方案


关于名称
- 使用 python 的 `from python_with_rust import my_rust_project` 时，这里的 `my_rust_project` 对应的是 `./my_rust_project/src/lib.rs` 中 `#[pymodule]` 定义的那个函数
- `setup.py` 中的 `RustExtension("python_with_rust.my_rust_project")` 对应的是安装后的文件夹名、.so 文件名。为了防止混乱，最好取值为 python 项目名、Rust 项目名