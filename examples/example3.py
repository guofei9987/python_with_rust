from python_with_rust import my_rust_project

print("get str from Rust:", my_rust_project.get_str())
print("get i32 from Rust:", my_rust_project.get_i32())
print("get f64 from Rust:", my_rust_project.get_f64())
print("get list from Rust:", my_rust_project.get_list())
print("get list from Rust:", my_rust_project.get_list_of_string())
print("get list from Rust:", my_rust_project.get_list_of_float())

# %% 把 Python 的 list、str、int、float 类型传递给 Rust

my_rust_project.accept_python_types(
    [1, 2, 3, "123"],  # list
    "Hello, Rust!",  # string
    42,  # integer
    3.14159  # float
)


# %%

# 把一个 Python 对象传进去


class MyPyCls:
    def __init__(self):
        self.val1 = 1
        self.val2 = "hello"
        self.val3 = [1, 2, 3]
        self.val4 = ['你好', 'Rust']


my_py_obj = MyPyCls()

my_rust_project.accept_python_object(
    my_py_obj
)

my_object = {
    "key1": "value1",
    "key2": "value2"
}

# 将 Python 对象传递给 Rust 函数
my_rust_project.accept_python_object(my_object)
