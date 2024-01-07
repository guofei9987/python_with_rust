use pyo3::prelude::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn my_rust_fn() {
    println!("调用了 Rust 函数 my_rust_fn");
}


#[pyfunction]
pub fn my_rust_fn2() {
    println!("调用了 Rust 函数 my_rust_fn");
}

#[pymodule]
fn rust_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(my_rust_fn2,m)?)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
