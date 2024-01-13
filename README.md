
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
python examples/example3.py # 测试相互传递 list、string 等数据
python examples/example4.py # 测试可以调用 Rust 类
```

## 三、说明

- `./my_rust_project1` 是一个纯 Rust 项目
- `./my_rust_project` 调用 `my_rust_project1`，并使用 pyo3 使其可以被 Python 调用。它相当于 “中间协调层”
    - 也可以不要这一层，而是直接编译 `my_rust_project1`，然后用 python 的 ctypes 来调用，实际上是 ffi 方法。这种方法参见 [郭飞的笔记](https://www.guofei.site/2022/08/28/rust2.html#Python%20%E8%B0%83%E7%94%A8%20Rust%20%E7%BC%96%E8%AF%91%E5%90%8E)
    - 使用 ffi 的缺点是需要自定义数据类型，并有内存泄露的风险
    - 使用 ffi 的优点是某些情况下潜在性能更高，在 Rust 项目较小的情况下才推荐
    - 这个项目展示使用 `pyo3` 的方案
- python 调用 `my_rust_project1`，从而间接调用了 `my_rust_project`