from python_with_rust import python_with_rust


print("get str from Rust:", python_with_rust.get_str())
print("get i32 from Rust:", python_with_rust.get_i32())
print("get f64 from Rust:", python_with_rust.get_f64())
print("get list from Rust:", python_with_rust.get_list())
print("get list from Rust:", python_with_rust.get_list_of_string())
print("get list from Rust:", python_with_rust.get_list_of_float())

# %% 把 Python 的 list、str、int、float 类型传递给 Rust

python_with_rust.accept_python_types(
    [1, 2, 3],  # list
    {"a": 1, "b": 9},
    "Hello, Rust!",  # string
    42,  # integer
    3.14159  # float
)
