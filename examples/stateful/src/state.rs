use std::cell::RefCell;

use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub type StateListener = dyn Fn() + Send + Sync;

struct InnerState {
    name: String,
    listeners: Vec<Option<Box<StateListener>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateListenerHandle(usize);

#[derive(Clone)]
pub struct State(Arc<Mutex<InnerState>>);

impl State {
    pub fn name(&self) -> String {
        self.0.lock().unwrap().name.clone()
    }

    pub fn new(name: String) -> Self {
        State(Arc::new(Mutex::new(InnerState {
            name,
            listeners: Vec::new(),
        })))
    }

    pub fn set_name(&mut self, new_name: String) {
        self.0.lock().unwrap().name = new_name;
        self.call_listeners();
    }

    pub fn add_listener(&mut self, listener: impl Into<Box<StateListener>>) -> StateListenerHandle {
        let mut inner_state = self.0.lock().unwrap();
        inner_state.listeners.push(Some(listener.into()));

        StateListenerHandle(inner_state.listeners.len() - 1)
    }

    pub fn remove_listener(&mut self, handle: StateListenerHandle) {
        let mut inner_state = self.0.lock().unwrap();
        if let Some(value) = inner_state.listeners.get_mut(handle.0) {
            value.take();
        }
    }

    fn call_listeners(&self) {
        let inner_state = self.0.lock().unwrap();
        inner_state
            .listeners
            .iter()
            .filter(|listener| listener.is_some())
            .for_each(|listener| {
                (listener.as_ref().unwrap())();
            })
    }
}

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref STATE: State = State::new(String::from("Name"));
}
