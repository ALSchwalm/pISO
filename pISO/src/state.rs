use displaymanager::{DisplayManager, Widget};
use error::{Result, ResultExt};
use serde;
use serde_json;
use std::fs::File;
use std::path::PathBuf;

pub trait State {
    fn index(&self) -> Option<String> {
        None
    }
    fn store(&self) -> Result<serde_json::Value> {
        Err("Attempt to store a non-stateful widget".into())
    }
    fn load(&mut self, _: serde_json::Value) -> Result<()> {
        Ok(())
    }
    fn after_load(&mut self, &mut DisplayManager) -> Result<()> {
        Ok(())
    }
}

pub trait Stateful {
    type State: serde::Serialize + serde::de::DeserializeOwned;
    fn state(&self) -> &Self::State;
    fn state_mut(&mut self) -> &mut Self::State;
    fn key(&self) -> String;
    fn on_load(&mut self, &mut DisplayManager) -> Result<()>;
}

impl<T, U> State for T
where
    T: Stateful<State = U>,
    U: serde::Serialize + serde::de::DeserializeOwned,
{
    fn store(&self) -> Result<serde_json::Value> {
        serde_json::to_value(self.state()).chain_err(|| "Failed to serialize state")
    }

    fn load(&mut self, value: serde_json::Value) -> Result<()> {
        *self.state_mut() = serde_json::from_value(value)?;
        Ok(())
    }

    fn after_load(&mut self, disp: &mut DisplayManager) -> Result<()> {
        self.on_load(disp)
    }

    fn index(&self) -> Option<String> {
        Some(self.key())
    }
}

pub struct StateManager {
    pub path: PathBuf,
}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            path: "/boot/piso.state".into(),
        }
    }

    fn current_state(&mut self) -> serde_json::Value {
        let f = File::open(&self.path).unwrap();
        serde_json::from_reader(f).unwrap()
    }

    pub fn load_state(&mut self, root: &mut Widget, disp: &mut DisplayManager) -> Result<()> {
        let state = self.current_state();
        fn visit(
            widget: &mut Widget,
            disp: &mut DisplayManager,
            state: &serde_json::Value,
        ) -> Result<()> {
            if let Some(key) = widget.index() {
                if let Some(loaded) = state.get(key) {
                    widget.load(loaded.clone())?;
                }
                widget.after_load(disp)?;
            }
            for child in widget.mut_children() {
                visit(child, disp, state)?;
            }
            Ok(())
        }
        visit(root, disp, &state)
    }

    pub fn save_state(&mut self, root: &mut Widget) -> Result<()> {
        let mut state = self.current_state();
        let old_state = state.clone();
        fn visit(widget: &mut Widget, state: &mut serde_json::Value) -> Result<()> {
            if let Some(key) = widget.index() {
                state[key] = widget.store()?;
            }
            for child in widget.mut_children() {
                visit(child, state)?;
            }
            Ok(())
        }
        visit(root, &mut state)?;
        if state != old_state {
            let mut f = File::open(&self.path).unwrap();
            serde_json::ser::to_writer(&mut f, &state)?;
        }
        Ok(())
    }
}
