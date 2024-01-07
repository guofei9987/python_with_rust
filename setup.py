from setuptools import setup
from setuptools_rust import Binding, RustExtension


setup(
    name='python_with_rust',
    version='0.0.1',
    python_requires='>=3.5',
    packages=['python_with_rust'],
    rust_extensions=[
        RustExtension("python_with_rust.my_rust_project",
                      path="./my_rust_project/Cargo.toml", binding=Binding.PyO3)
    ],
)
