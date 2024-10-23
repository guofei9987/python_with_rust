use std::collections::HashMap;
use pyo3::prelude::*;
use my_rust_project1;
use pyo3::types::{PyFloat, PyInt, PyList, PyString};
use pyo3::types::{PyDict, PyAny};

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

#[pyfunction]
fn get_str() -> PyResult<String> {
    let rust_string = "Hello from Rust!".to_string();
    Ok(rust_string)
}


#[pyfunction]
fn get_i32() -> PyResult<i32> {
    let rust_i32 = 42;
    Ok(rust_i32)
}

#[pyfunction]
fn get_f64() -> PyResult<f64> {
    let rust_f64 = 3.14;
    Ok(rust_f64)
}


#[pyfunction]
fn get_list() -> PyResult<PyObject> {
    let rust_vec = vec![1, 2, 3];
    Python::with_gil(|py| {
        let python_list = PyList::new_bound(py, &rust_vec);
        Ok(python_list.to_object(py))
    })
}


// 报错：
#[pyfunction]
fn get_list_of_string() -> PyResult<PyObject> {
    let rust_vec_of_string = vec!["string1".to_string(), "string2".to_string(), "string3".to_string()];
    Python::with_gil(|py| {
        let python_list = PyList::new_bound(py, rust_vec_of_string);
        Ok(python_list.to_object(py))
    })
}


#[pyfunction]
fn get_list_of_float() -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let rust_vec_of_float = vec![1.0, 2.5, 3.14];
        let python_list = PyList::new_bound(py, rust_vec_of_float);
        Ok(python_list.to_object(py))
    })
}


// 从 Python 接收参数
#[pyfunction]
fn accept_python_types(
    list: Vec<i32>,
    dict: HashMap<String, f32>,
    string: String,
    integer: i32,
    float: f64,
) -> PyResult<()> {
    println!(r"
    调用了 Rust 函数：[accept_python_types]
    接收了:
    list = {:?}
    dict = {:?}
    String = {},
    i32 = {},
    f64 = {}", list, dict, string, integer, float);
    Ok(())
}


#[pyfunction]
fn accept_python_object(dict: HashMap<String, f32>) -> PyResult<()> {
    // // 提取字典中的某个键的值并转换为 String
    // if let Some(value) = dict.get_item("key") {
    //     let extracted_value: String = value.extract()?;
    //     return Ok(format!("Received from Python dict: {}", extracted_value));
    // }
    //
    // Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>("Key not found"))

    println!("传入 dict = {:?}", dict);
    Ok(())
}

// 一个类

#[pyclass]
struct MyRecorder {
    count: usize,
}

#[pymethods]
impl MyRecorder {
    #[new]
    fn new() -> Self {
        MyRecorder { count: 0 }
    }

    fn func1(&mut self) {
        self.count += 1;
    }

    fn func2(&self) -> usize {
        self.count
    }
}


#[pymodule]
fn python_with_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // 这个函数名 my_rust_project 一定要按需修改，否则 import 的时候找不到
    m.add_function(wrap_pyfunction!(my_rust_fn2, m)?)?;
    m.add_function(wrap_pyfunction!(get_str, m)?)?;
    m.add_function(wrap_pyfunction!(get_i32, m)?)?;
    m.add_function(wrap_pyfunction!(get_f64, m)?)?;
    m.add_function(wrap_pyfunction!(get_list, m)?)?;
    m.add_function(wrap_pyfunction!(get_list_of_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_list_of_float, m)?)?;
    m.add_function(wrap_pyfunction!(accept_python_types, m)?)?;
    m.add_function(wrap_pyfunction!(accept_python_object, m)?)?;
    m.add_class::<MyRecorder>()?;
    Ok(())
}