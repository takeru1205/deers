use pyo3::types::PyDict;
use pyo3::{exceptions::PyValueError, prelude::*};
use std::collections::HashMap;

#[pyclass]
pub struct Graph {
    graph: HashMap<String, String>,
    _node: HashMap<i32, HashMap<String, i32>>,
    _adj: HashMap<i32, HashMap<String, i32>>,
}

#[pymethods]
impl Graph {
    #[new]
    #[pyo3(signature = (**py_kwargs))]
    fn new(py_kwargs: Option<&PyDict>) -> Self {
        let mut g = HashMap::new();
        if py_kwargs.is_some() {
            for (k, v) in py_kwargs
                .unwrap()
                .keys()
                .iter()
                .zip(py_kwargs.unwrap().values().iter())
            {
                g.insert(format!("{}", k), format!("{}", v));
            }
        };

        Graph {
            graph: g,
            _node: HashMap::new(),
            _adj: HashMap::new(),
        }
    }

    fn add_node(&mut self, node_for_adding: Option<i32>) -> PyResult<()> {
        if !self._node.contains_key(&node_for_adding.unwrap()) {
            if node_for_adding.is_none() {
                return Err(PyValueError::new_err("Value Error: None cannot be a node"));
            }
            self._adj.insert(node_for_adding.unwrap(), HashMap::new());
            self._node.insert(node_for_adding.unwrap(), HashMap::new());
        };
        Ok(())
    }

    /*
    fn __str__<'p>(&self, py: Python<'p>) -> PyResult<PyString> {
        let mut print_statement = String::new();
        for (key, value) in &self._node {
            println!("{}: {:?}", key, value);
            print_statement = print_statement + &format!("{}: {:?}", key, value);
        }
        Ok(PyString::new(py, &print_statement))
    }
    */
    fn str(&self) -> PyResult<()> {
        for (key, value) in &self._node {
            println!("{}: {:?}", key, value);
        }
        Ok(())
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.graph.get("name").unwrap().to_string())
    }

    fn __len__(&self) -> usize {
        return self._node.len() as usize;
    }
}
