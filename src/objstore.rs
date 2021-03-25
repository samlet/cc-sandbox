use pyo3::prelude::*;

#[pyclass]
#[derive(Default)]
pub struct ObjStore {
    obj: Vec<PyObject>,
}

#[pymethods]
impl ObjStore {
    #[new]
    fn new() -> Self {
        ObjStore::default()
    }

    fn push(&mut self, py: Python, obj: &PyAny) {
        self.obj.push(obj.to_object(py));
    }
}

#[pymodule]
fn objstore(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<ObjStore>()
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use pyo3::types::{IntoPyDict, PyDateTime, PyString};

    #[test]
    fn it_works() -> anyhow::Result<()> {
        Ok(())
    }

    #[test]
    fn pydelta_conversion() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let datetime = py.import("datetime").unwrap();
        let locals = [("datetime", datetime)].into_py_dict(py);
        let now: &PyDateTime = py
            .eval("datetime.datetime.utcnow()", None, Some(&locals))
            .unwrap()
            .downcast()
            .unwrap();
        println!("{:?}", now);
    }

    #[test]
    fn pandas_works() -> anyhow::Result<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let _pandas = py.import("pandas").unwrap();
        Ok(())
    }

    #[test]
    fn sagas_works() -> anyhow::Result<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let sagas = py.import("sagas").unwrap();
        let locals = [("sagas", sagas)].into_py_dict(py);
        let result: &PyString = py
            .eval("sagas.util.name_util.to_global_id('x','y')",
                  None, Some(&locals))
            .unwrap()
            .downcast()
            .unwrap();
        println!("{:?}", result);
        Ok(())
    }
}



