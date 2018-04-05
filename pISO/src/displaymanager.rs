use action;
use bitmap;
use controller;
use display;
use error::{Result, ResultExt};
use error_chain::ChainedError;
use input;
use render;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub type WindowId = u32;

pub enum Position {
    Fixed(usize, usize),
    Normal,
}

pub struct Window {
    pub position: Position,
    pub z: u32,
    pub focus: bool,
    pub id: WindowId,
    pub bitmap: bitmap::Bitmap,
}

pub struct DisplayManager {
    display: display::Display,
    windows: BTreeMap<WindowId, Window>,
    nextid: u32,
}

impl DisplayManager {
    pub fn new() -> Result<Arc<Mutex<DisplayManager>>> {
        let mut disp = display::Display::new().chain_err(|| "Failed to create display")?;
        disp.on().chain_err(|| "Failed to activate display")?;

        Ok(Arc::new(Mutex::new(DisplayManager {
            display: disp,
            windows: BTreeMap::new(),
            nextid: 1,
        })))
    }

    pub fn add_child(&mut self, pos: Position) -> Result<WindowId> {
        let id = self.nextid;
        self.nextid += 1;

        self.windows.insert(
            id,
            Window {
                position: pos,
                id: id,

                //TODO: this should probably be the parent z + 1
                z: 0,
                focus: false,
                bitmap: bitmap::Bitmap::new(0, 0),
            },
        );

        Ok(id)
    }

    pub fn remove_child(&mut self, id: WindowId) -> Result<()> {
        //TODO: deal with focus
        self.windows.remove(&id);
        Ok(())
    }

    pub fn get(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }

    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    fn parent_widget<'a>(root: &Widget, target: WindowId) -> Option<&Widget> {
        fn visit<'a>(current: &Widget, target: WindowId) -> Option<&Widget> {
            if current.windowid() == target {
                return None;
            } else {
                for child in current.children() {
                    if child.windowid() == target {
                        return Some(current);
                    }
                    let res = visit(child, target);
                    if res.is_some() {
                        return res;
                    }
                }
                return None;
            }
        }
        visit(root, target)
    }

    fn normal_position(&self, root: &Widget, widget: &Widget) -> (usize, usize) {
        fn visit(
            disp: &DisplayManager,
            current: &Widget,
            target: &Widget,
            mut pos: (usize, usize),
        ) -> (bool, usize, usize) {
            if current.windowid() == target.windowid() {
                (true, pos.0, pos.1)
            } else {
                let window = disp.get(target.windowid()).expect("widget has no window");
                let height = match window.position {
                    Position::Fixed(_, _) => 0,
                    Position::Normal => window.bitmap.height(),
                };
                pos = (pos.0, pos.1 + height);
                for child in current.children() {
                    let res = visit(disp, child, target, pos);
                    if res.0 {
                        return res;
                    } else {
                        pos = (res.1, res.2);
                    }
                }
                (false, pos.0, pos.1)
            }
        }
        let res = visit(self, root, widget, (0, 0));
        if !res.0 {
            panic!("Unable to find widget with id={}", widget.windowid());
        } else {
            (res.1, res.2)
        }
    }

    fn calculate_position(&self, root: &Widget, widget: &Widget) -> (usize, usize) {
        let window = self.get(widget.windowid()).expect("widget has no window");

        match window.position {
            Position::Fixed(x, y) => (x, y),
            Position::Normal => self.normal_position(root, widget),
        }
    }

    fn first_descendant(widget: &Widget) -> &Widget {
        widget
            .children()
            .first()
            .map(|child| Self::first_descendant(*child))
            .unwrap_or(widget)
    }

    fn last_descendant(widget: &Widget) -> &Widget {
        widget
            .children()
            .last()
            .map(|child| Self::first_descendant(*child))
            .unwrap_or(widget)
    }

    fn next_widget<'a>(root: &Widget, target: WindowId) -> Option<&Widget> {
        fn visit<'a>(current: &Widget, target: WindowId) -> (bool, Option<&Widget>) {
            if current.windowid() == target {
                return (true, current.children().first().map(|n| *n));
            } else {
                for (idx, child) in current.children().iter().enumerate() {
                    let res = visit(*child, target);
                    if res.0 {
                        if res.1.is_none() {
                            return (true, current.children().get(idx + 1).map(|n| *n));
                        } else {
                            return res;
                        }
                    }
                }
                return (false, None);
            }
        }
        visit(root, target).1
    }

    fn prev_widget<'a>(root: &Widget, target: WindowId) -> Option<&Widget> {
        if let Some(parent) = Self::parent_widget(root, target) {
            let children = parent.children();
            children
                .iter()
                .position(|child| target == child.windowid())
                .and_then(|pos| children.into_iter().nth(pos - 1))
                .map(|widget| Self::last_descendant(widget))
                .or(Some(parent))
        } else {
            None
        }
    }

    fn find_mut_widget(root: &mut Widget, target: WindowId) -> Option<&mut Widget> {
        if root.windowid() == target {
            return Some(root);
        } else {
            for child in root.mut_children() {
                let res = DisplayManager::find_mut_widget(child, target);
                if res.is_some() {
                    return res;
                }
            }
        }
        None
    }

    pub fn shift_focus(&mut self, widget: &Widget) {
        println!("Shifting focus to window id={}", widget.windowid());
        for (&id, cand) in self.windows.iter_mut() {
            if id == widget.windowid() {
                cand.focus = true;
            } else {
                cand.focus = false;
            }
        }
    }

    fn find_focused_widget<'a>(&self, current: &'a mut Widget) -> Option<&'a mut Widget> {
        let focus = self.get(current.windowid())
            .expect("Unable to find focused window")
            .focus;
        if focus {
            return Some(current);
        }
        for child in current.mut_children().into_iter() {
            if let Some(child) = self.find_focused_widget(child) {
                return Some(child);
            }
        }
        None
    }

    pub fn on_event(
        &mut self,
        root: &mut Widget,
        event: &controller::Event,
    ) -> Result<Vec<action::Action>> {
        let focus_id = {
            let focused = self.find_focused_widget(root).expect("No focused window");
            let mut event_res = focused.on_event(event)?;
            if event_res.0 {
                return Ok(event_res.1);
            } else {
                focused.windowid()
            }
        };

        match event {
            &controller::Event::Up => {
                if let Some(prev) = DisplayManager::prev_widget(root, focus_id) {
                    // Special case, do not focus the root
                    if prev.windowid() != root.windowid() {
                        self.shift_focus(prev);
                    }
                } else {
                    println!("No previous widget for window id={}", focus_id);
                }
            }
            &controller::Event::Down => {
                if let Some(next) = DisplayManager::next_widget(root, focus_id) {
                    self.shift_focus(next);
                } else {
                    println!("No next widget for window id={}", focus_id);
                }
            }
            _ => (),
        }

        Ok(vec![])
    }

    pub fn do_actions(
        &mut self,
        root: &mut Widget,
        actions: &mut Vec<action::Action>,
    ) -> Result<()> {
        let mut new_actions = vec![];
        fn visit(
            manager: &mut DisplayManager,
            widget: &mut Widget,
            actions: &mut Vec<action::Action>,
            new_actions: &mut Vec<action::Action>,
        ) -> Result<()> {
            if actions.len() == 0 {
                return Ok(());
            }
            actions.retain(|action| match widget.do_action(manager, action) {
                Ok((handled, new)) => {
                    new_actions.extend(new);
                    !handled
                }
                Err(e) => {
                    println!(
                        "Error while processing '{:?}': {}",
                        action,
                        e.display_chain()
                    );
                    false
                }
            });

            for child in widget.mut_children() {
                visit(manager, child, actions, new_actions)?;
            }

            Ok(())
        }

        visit(self, root, actions, &mut new_actions)?;
        actions.extend(new_actions);
        Ok(())
    }

    // First, render everything so we know the sizes to position things
    fn do_render(&mut self, root: &Widget) -> Result<()> {
        fn visit(manager: &mut DisplayManager, widget: &Widget) -> Result<()> {
            {
                let window = manager
                    .get_mut(widget.windowid())
                    .ok_or(format!("failed to find window id={}", widget.windowid()))?;
                window.bitmap = widget.render(window)?;
            }

            for child in widget.children() {
                visit(manager, child)?;
            }

            Ok(())
        }

        visit(self, root)
    }

    pub fn render(&mut self, root: &Widget) -> Result<()> {
        self.do_render(root)?;

        fn position_window(
            manager: &mut DisplayManager,
            base: &mut bitmap::Bitmap,
            root: &Widget,
            widget: &Widget,
        ) -> Result<()> {
            println!("Positioning windowid={}", widget.windowid());
            //TODO: make this less terrible
            let pos = manager.calculate_position(root, widget);
            {
                let mut window = manager
                    .get_mut(widget.windowid())
                    .ok_or(format!("failed to find window id={}", widget.windowid()))?;

                println!(
                    "Blitting to ({}, {}, size=({}, {}))",
                    pos.0,
                    pos.1,
                    window.bitmap.width(),
                    window.bitmap.height()
                );
                base.blit(&window.bitmap, pos);
            }

            for child in widget.children() {
                position_window(manager, base, root, child)?
            }

            Ok(())
        };

        let mut bitmap = bitmap::Bitmap::new(0, 0);
        position_window(self, &mut bitmap, root, root)?;

        println!(
            "Update display with bitmap: {} by {}",
            bitmap.width(),
            bitmap.height()
        );
        self.display.update(bitmap)?;

        Ok(())
    }
}

pub trait Widget: render::Render + input::Input {
    fn mut_children(&mut self) -> Vec<&mut Widget>;
    fn children(&self) -> Vec<&Widget>;
    fn windowid(&self) -> WindowId;
}
