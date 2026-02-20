use pyo3::prelude::*;
use std::time::Duration;


use crossbeam::channel::{unbounded, Receiver, RecvTimeoutError};
use noot_core::state::{BackpackContents, EngineerProgress, FuelTanks, GameState, Module, ModuleEngineering, ModuleModifier, PowerplayInfo, Suit, SuitLoadout};
use noot_core::watcher::JournalWatcher;
use noot_core::JournalEvent;
use pyo3::exceptions::PyRuntimeError;
use pyo3::intern;
use pyo3::types::PyTuple;
use pythonize::pythonize;

#[pyclass(name = "Watcher")]
pub struct PyJournalWatcher {
    inner: JournalWatcher,
    rx: Receiver<anyhow::Result<(GameState, JournalEvent)>>
}

#[pymethods]
impl PyJournalWatcher {
    #[new]
    fn py_new(paths: Vec<String>) -> Self {
        let (tx, rx) = unbounded();
        let mut journal = JournalWatcher::new(tx);
        for p in paths {
            journal.add(p.as_str());
        }
        Self {
            inner: journal,
            rx
        }
    }

    fn watch<'py>(&self, py: Python<'py>, timeout_ms: u64, stop_event: Py<PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let stop_event_is_set: Option<Bound<PyAny>> = match stop_event.is_none(py) {
            true => None,
            false => Some(stop_event.getattr(py, intern!(py, "is_set"))?.into_bound(py)),
        };
        if let Some(is_set) = &stop_event_is_set {
            if is_set.call0()?.is_truthy()? {
                return Ok(intern!(py, "stop").as_any().to_owned());
            }
        }
        let recv_result = py.detach(|| self.rx.recv_timeout(Duration::from_millis(timeout_ms)));
        return match recv_result {
            Ok(Ok((state, event))) => {
                let state = state.into_pyobject(py)?;
                let event = pythonize(py, &event)?;
                let tuple = PyTuple::new(py, &[state.as_any(), &event])?;
                Ok(tuple.into_any())
            }
            Ok(Err(e)) => {
                Err(PyRuntimeError::new_err(e.to_string()))
            }
            Err(RecvTimeoutError::Timeout) => {
                py.check_signals()?;
                Ok(intern!(py, "timeout").as_any().to_owned())
            }
            Err(RecvTimeoutError::Disconnected) => Ok(py.None().into_bound(py)),
        }
    }

    fn __enter__(mut slf: PyRefMut<Self>) -> PyResult<PyRefMut<Self>> {
        slf.inner.start();
        Ok(slf)
    }

    fn __exit__(&mut self, _exc_type: Py<PyAny>, _exc_value: Py<PyAny>, _traceback: Py<PyAny>) {
        self.inner.stop(false);
    }
}


#[pymodule(gil_used = false)]
pub fn _rust_watcher(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    let mut version = env!("CARGO_PKG_VERSION").to_string();
    // cargo uses "1.0-alpha1" etc. while raxxla uses "1.0.0a1", this is not full compatibility,
    // but it's good enough for now
    // see https://docs.rs/semver/1.0.9/semver/struct.Version.html#method.parse for rust spec
    // see https://peps.python.org/pep-0440/ for raxxla spec
    // it seems the dot after "alpha/beta" e.g. "-alpha.1" is not necessary, hence why this works
    version = version.replace("-alpha", "a").replace("-beta", "b");
    m.add("__version__", version)?;
    m.add_class::<PyJournalWatcher>()?;
    m.add_class::<GameState>()?;
    m.add_class::<EngineerProgress>()?;
    m.add_class::<PowerplayInfo>()?;
    m.add_class::<BackpackContents>()?;
    m.add_class::<SuitLoadout>()?;
    m.add_class::<Suit>()?;
    m.add_class::<Module>()?;
    m.add_class::<FuelTanks>()?;
    m.add_class::<ModuleEngineering>()?;
    m.add_class::<ModuleModifier>()?;
    Ok(())
}
