
# python_with_rust

演示 Python 如何调用一个 Rust 项目
1. Python 传给 Rust 这些数据结构：str, int, float, list, dict 等
2. Rust 返回给 Python 这些数据结构：String, i32, f64, Vec<i32>, Vec<String>
3. Python 使用 Rust 类
4. 把 Python 项目发布，并且兼容多种操作系统


说明
- 自从 [PEP 518](https://peps.python.org/pep-0518/) 以来，`pyproject.toml` 是比 `setup.py` 更为先进的构建文件。
- 使用 `setup.py` 管理方式见于分支 [use_setup.py](https://github.com/guofei9987/python_with_rust/tree/use_setup.py)，main 分支使用 `pyproject.toml`


## 一、安装

两种方式：
1. 使用 `pip install python_with_rust==0.1.5` 但只限于已经对对应的环境预编译的情况，包含：
    - `ubuntu-latest`, `macos-latest`, `windows-latest`; python 版本为：'3.7', '3.8', '3.9', '3.10', '3.11' ,'3.12'
    - 使用 CI 方法，在 **release** 时从 pypi 针对一些环境预编译并发布，CI见于[workflows/release.yml](workflows/release.yml)
    - pypi 托管的预编译文件可以查看 [https://pypi.org/project/numpy/#files](https://pypi.org/project/numpy/#files)
2. 如果环境不在上述范围内，可以下载项目后在本地编译并安装。包括以下情况：
    - macos x86。Intel 系列的 Macbook，或者虽然是 M 系列处理器，但是 python 环境是通过 Rosetta 2 以 Intel 模拟模式运行的。
    - manylinux/musllinux 的 aarch64版本。一般出现在移动设备、嵌入式设备上，或者用 MacBook 打开的默认 docker
    - Linux 的 glibc 版本较老的
    - win32
    - 上述情况实际上也可以用 CI 方法批量向 pypi 发布。但此项目只是示例，没必要都涵盖。
    - `uname -m` 查询架构， `python -c "import platform; print(platform.machine())"` 查询环境



如何查询自己的环境？
- `uname -m` 查询架构
- `python -c "import platform; print(platform.machine())"` 查询 Python 环境
- `ldd --version` 查询 glibc 版本

本地编译并安装的步骤：
1. 安装 rust、Python（略），可以升级到较新版本
2. 安装 maturin `pip install maturin`
3. 在本地编译并安装此项目 `matruin develop`


## 二、测试

```shell
python examples/example1.py # 测试可以调用 Python 函数
python examples/example2.py # 测试可以调用 Rust 函数
python examples/example3.py # 测试相互传递 list、string 等数据
python examples/example4.py # 测试可以调用 Rust 类
```

## 三、说明

- `./my_rust_project1` 是一个纯 Rust 项目，是 python 所要调用的 rust项目。它与 `pyo3` 之类的无关。
- `./Cargo.toml` 是一个中间层，它使用 pyo3，调用 `my_rust_project1` 并被 Python 调用
- 另一个方案是 ffi 方法，使用 C 标准编译 `my_rust_project1`，编译后的代码可以被 Python(ctypes)/C/Rust/Java 调用。具体参见 [郭飞的笔记](https://www.guofei.site/2022/08/28/rust2.html#Python%20%E8%B0%83%E7%94%A8%20Rust%20%E7%BC%96%E8%AF%91%E5%90%8E)
    - 缺点是需要自定义数据类型，并有内存泄露的风险。优点是某些情况下潜在性能更高
    - 使用 `pyo3` 是以一个比较好的实践，这个项目仅展示使用 `pyo3` 的方案

