use displaymanager::{DisplayManager, Widget};
use error::{Result, ResultExt};
use serde;
use serde_json;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Mutex;

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
    pub state: serde_json::Value,
}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            path: "/boot/piso.state".into(),
            state: json!({}),
        }
    }

    fn read_state(&mut self) -> serde_json::Value {
        match File::open(&self.path) {
            Ok(f) => serde_json::from_reader(f).expect("Failed to load state"),
            Err(_) => json!({}),
        }
    }

    pub fn get<I: serde_json::value::Index, S: serde::de::DeserializeOwned>(
        &self,
        index: I,
    ) -> Result<S> {
        Ok(serde_json::from_value(
            self.state.get(index).ok_or("Failed to get state")?.clone(),
        )?)
    }

    pub fn load_state(&mut self, root: &mut Widget, disp: &mut DisplayManager) -> Result<()> {
        self.state = self.read_state();
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
        visit(root, disp, &self.state)
    }

    pub fn save_state(&mut self, root: &mut Widget) -> Result<()> {
        let old_state = self.state.clone();
        {
            let mut values = self.state.as_object_mut().ok_or("State is not an object")?;
            fn visit(
                widget: &mut Widget,
                state: &mut serde_json::Map<String, serde_json::Value>,
            ) -> Result<()> {
                if let Some(key) = widget.index() {
                    state.insert(key, widget.store()?);
                }
                for child in widget.mut_children() {
                    visit(child, state)?;
                }
                Ok(())
            }
            visit(root, &mut values)?;
        }
        if self.state != old_state {
            let mut f = File::create(&self.path)?;
            serde_json::ser::to_writer(&mut f, &self.state)?;
        }
        Ok(())
    }
}

lazy_static! {
    pub static ref PERSISTENT_STATE: Mutex<StateManager> = { Mutex::new(StateManager::new()) };
}
