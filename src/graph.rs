use pyo3::types::PyBytes;
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    types::{PyTuple, PyType},
};
use std::collections::HashMap;

pub struct node {
    node: i32,
}

#[pyclass]
#[derive(Clone)]
pub struct Graph {
    graph: HashMap,
    _node: HashMap,
    _adj: HashMap,
    number_of_nodes: i32,
    number_of_edges: i32,
    name: &str,
}
