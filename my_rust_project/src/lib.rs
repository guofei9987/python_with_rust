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
        let python_list = PyList::new(py, &rust_vec);
        Ok(python_list.to_object(py))
    })
}


#[pyfunction]
fn get_list_of_string() -> PyResult<PyObject> {
    let rust_vec_of_string = vec!["string1".to_string(), "string2".to_string(), "string3".to_string()];
    Python::with_gil(|py| {
        let python_list = PyList::new(py, rust_vec_of_string);
        Ok(python_list.to_object(py))
    })
}


#[pyfunction]
fn get_list_of_float() -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let rust_vec_of_float = vec![1.0, 2.5, 3.14];
        let python_list = PyList::new(py, rust_vec_of_float);
        Ok(python_list.to_object(py))
    })
}


// 从 Python 接收参数
#[pyfunction]
fn accept_python_types(
    list: &PyList,
    string: &PyString,
    integer: &PyInt,
    float: &PyFloat,
) -> PyResult<()> {
    println!("调用了 Rust 函数：accept_python_types，这个函数从 Python 接收 list、str、int、float 类型");

    for item in list.iter() {
        println!("List item: {:?}", item);
    }

    let rust_string = string.to_str()?;
    println!("String: {}", rust_string);

    let rust_int = integer.extract::<i64>()?;
    println!("Integer: {}", rust_int);

    let rust_float = float.extract::<f64>()?;
    println!("Float: {}", rust_float);

    println!("函数调用完成\n\n");
    Ok(())
}


// 把 python 字典传进来
#[pyfunction]
fn accept_python_object(obj: &PyAny) -> PyResult<()> {
    println!("Rust 接收一个 Python对象");
    // 检查传入的对象是否是一个字典
    if let Ok(dict) = obj.downcast::<PyDict>() {
        // 使用字典
        for (key, value) in dict {
            println!("{}: {}", key, value);
        }
    } else {
        // 对象不是字典，处理其它情况
        println!("The object is not a dict!");
    }

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
fn my_rust_project(_py: Python, m: &PyModule) -> PyResult<()> {
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