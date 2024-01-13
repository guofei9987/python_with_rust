
# python_with_rust

演示 Python 如何调用一个 Rust 项目

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
```

## 三、说明

- `./my_rust_project1` 是一个纯 Rust 项目
- `./my_rust_project` 使用 pyo3 调用 `my_rust_project1`，相当于一个中间的 “协调层”
    - 也可以不要这一层，不用 pyo3，而是直接编译 `my_rust_project1`，然后用 python 的 ctypes 来调用，实际上是 ffi 方法。
    - 使用 ffi 的缺点是需要自定义数据类型，并有内存泄露的风险
    - 使用 ffi 的优点是某些情况下潜在性能更高，在 Rust 项目较小的情况下才推荐
- python 调用 `my_rust_project`，从而间接调用了 `my_rust_project1`