use crate::adapter::{print_output, SearchIn};
use crate::file::FileMata;
use pyo3::types::{PyModule, PyTuple};
use pyo3::{Py, PyAny, PyResult, Python};
use std::path::PathBuf;

pub struct PdfFileType {
    file_meta: FileMata,
    pattern: String,
}

impl PdfFileType {
    pub fn new(file_meta: FileMata, pattern: String) -> Self {
        PdfFileType { file_meta, pattern }
    }
}

impl SearchIn for PdfFileType {
    fn search_in(&self) {
        pyo3::prepare_freethreaded_python();
        let abs_path = std::fs::canonicalize(&self.file_meta.path)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        // println!("{}", a);
        let py_pdf = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/adapter/pdf/pdf.py"
        ));
        let from_py = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            let app: Py<PyAny> = PyModule::from_code(py, py_pdf, "pdf", "pdf")?
                .getattr("extract_pdf")?
                .into();
            let args = PyTuple::new(py, &[abs_path]);
            app.call1(py, args)
        });
        let file_content = from_py.unwrap().to_string();
        print_output(
            file_content,
            String::from(&self.pattern),
            PathBuf::from(&self.file_meta.path),
        )
    }
}
