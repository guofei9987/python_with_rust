from setuptools import setup
from setuptools_rust import Binding, RustExtension
import os
# 配合使用：
this_directory = os.path.abspath(os.path.dirname(__file__))


# 读取 README
def read_file(filename):
    with open(os.path.join(this_directory, filename), encoding='utf-8') as f:
        long_description = f.read()
    return long_description


# 获取依赖
def read_requirements(filename):
    return [line.strip() for line in read_file(filename).splitlines()
            if not line.startswith('#')]


setup(
    name='python_with_rust',
    version='0.0.1',
    python_requires='>=3.5',
    packages=['python_with_rust'],
    install_requires=read_requirements('requirements.txt'),
    rust_extensions=[
        RustExtension("python_with_rust.my_rust_project",
                      path="./my_rust_project/Cargo.toml", binding=Binding.PyO3)
    ],
)
