use pyo3::prelude::*;
use my_rust_project1;

fn my_rust_fn1() {
    println!("调用了 Rust 函数：my_rust_fn1");
}


#[pyfunction]
fn my_rust_fn2() -> PyResult<()> {
    my_rust_project1::my_rust_fn1_1();
    my_rust_fn1();
    println!("调用了 Rust 函数：my_rust_fn2");
    Ok(())
}

#[pymodule]
fn my_rust_project(_py: Python, m: &PyModule) -> PyResult<()> {
    // 这个函数名 my_rust_project 一定要按需修改，否则 import 的时候找不到
    m.add_function(wrap_pyfunction!(my_rust_fn2, m)?)?;
    Ok(())
}