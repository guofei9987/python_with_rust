from setuptools import setup
from setuptools.command.install import install
import subprocess


class CustomInstallCommand(install):
    def run(self):
        #         在 安装之前构建 Rust 项目
        subprocess.check_call(['cargo', 'build', '--release'], cwd='./my_rust_project/')
        #         运行原有的 install 命令
        install.run(self)


setup(
    name='python_with_rust',
    version='0.0.1',
    python_requires='>=3.5',
    packages=['python_with_rust'],
    cmdclass={'install': CustomInstallCommand}
)
