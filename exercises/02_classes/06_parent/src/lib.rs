// TODO: Define a base class named `Account`, with a floating point `balance` property.
//  Then define a subclass named `AccountWithHistory`.
//  `AccountWithHistory` adds a `history` attribute: every time the `balance` is modified,
//  the old balance is stored in the `history` list. `history` can be accessed but not modified
//  directly. The `history` list should be initialized as an empty list.
use pyo3::prelude::*;

#[pyclass(subclass)]
struct Account {
    #[pyo3(set, get)]
    balance: f64,
}

#[pymethods]
impl Account {
    #[new]
    fn new(balance: f64) -> Self {
        Self { balance }
    }

    fn hello(&self) {}
}

#[pyclass(extends=Account)]
#[derive(Default)]
struct AccountWithHistory {
    #[pyo3(get)]
    history: Vec<f64>,
}

#[pymethods]
impl AccountWithHistory {
    #[new]
    fn new(balance: f64) -> PyClassInitializer<Self> {
        PyClassInitializer::from(Account::new(balance)).add_subclass(Self::default())
    }

    #[getter]
    fn balance(self_: PyRef<'_, Self>) -> f64 {
        self_.as_super().balance
    }

    #[setter]
    fn set_balance(mut self_: PyRefMut<'_, Self>, val: f64) {
        let old_val = self_.as_super().balance;
        self_.history.push(old_val);
        self_.as_super().balance = val;
    }
}

#[pymodule]
fn parent(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Account>().unwrap();
    m.add_class::<AccountWithHistory>().unwrap();
    Ok(())
}
