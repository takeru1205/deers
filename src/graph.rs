use pyo3::{exceptions::PyValueError, prelude::*};
use std::collections::HashMap;

#[pyclass]
pub struct Graph {
    graph: HashMap<i32, HashMap<String, i32>>,
    _node: HashMap<i32, HashMap<String, i32>>,
    _adj: HashMap<i32, HashMap<String, i32>>,
    number_of_nodes: i32,
    number_of_edges: i32,
    name: String,
}

#[pymethods]
impl Graph {
    #[new]
    fn new() -> Self {
        Graph {
            graph: HashMap::new(),
            _node: HashMap::new(),
            _adj: HashMap::new(),
            number_of_nodes: 0,
            number_of_edges: 0,
            name: String::new(),
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

    fn __len__(&self) -> usize {
        return self._node.len() as usize;
    }
}
