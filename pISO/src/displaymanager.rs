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

    fn parent_widget<'a>(root: &'a Widget, target: &'a Widget) -> Option<&'a Widget> {
        fn visit<'a>(current: &'a Widget, target: &'a Widget) -> Option<&'a Widget> {
            if current.windowid() == target.windowid() {
                return None;
            } else {
                for child in current.children() {
                    if child.windowid() == target.windowid() {
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

    fn next_widget<'a>(root: &'a Widget, widget: &'a Widget) -> Option<&'a Widget> {
        fn visit<'a>(current: &'a Widget, target: &'a Widget) -> (bool, Option<&'a Widget>) {
            if current.windowid() == target.windowid() {
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
        visit(root, widget).1
    }

    fn prev_widget<'a>(root: &'a Widget, widget: &'a Widget) -> Option<&'a Widget> {
        if let Some(parent) = Self::parent_widget(root, widget) {
            let children = parent.children();
            children
                .iter()
                .position(|child| widget.windowid() == child.windowid())
                .and_then(|pos| children.into_iter().nth(pos - 1))
                .map(|widget| Self::last_descendant(widget))
        } else {
            None
        }
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

    pub fn on_event(
        &mut self,
        root: &Widget,
        event: &controller::Event,
    ) -> Result<Vec<action::Action>> {
        let mut actions = vec![];
        enum VisitState {
            NotFound,
            FoundHandled,
            FoundNotHandled,
        }
        fn visit(
            manager: &mut DisplayManager,
            root: &Widget,
            widget: &Widget,
            event: &controller::Event,
            actions: &mut Vec<action::Action>,
        ) -> Result<VisitState> {
            let focus = manager
                .get(widget.windowid())
                .ok_or(format!("failed to find window id={}", widget.windowid()))?
                .focus;

            if focus {
                // If we have focus, try to handle the event
                let (handled, new_actions) = widget.on_event(event);
                actions.extend(new_actions);
                if handled {
                    println!("Focused window handled event");
                    return Ok(VisitState::FoundHandled);
                } else {
                    println!("Focused window did not handle event");
                    return Ok(VisitState::FoundNotHandled);
                }
            } else {
                // If we don't have focus, try to find out what does
                let mut res = (None, VisitState::NotFound);
                for child in widget.children().into_iter() {
                    match visit(manager, root, child, event, actions)? {
                        VisitState::FoundHandled => return Ok(VisitState::FoundHandled),
                        VisitState::FoundNotHandled => {
                            println!("Child in focus but did not handle event");
                            res = (Some(child), VisitState::FoundNotHandled);
                            break;
                        }
                        VisitState::NotFound => (),
                    }
                }

                //TODO: if one of our children was in focus but couldn't handle the event,
                //      shift focus to the 'next' window
                match (res.0, res.1, event) {
                    (Some(child), VisitState::FoundNotHandled, &controller::Event::Down) => {
                        if let Some(next) = DisplayManager::next_widget(root, child) {
                            manager.shift_focus(next);
                        } else {
                            manager.shift_focus(widget);
                        }
                        //TODO: if we can't take focus, propagate to our parent
                        return Ok(VisitState::FoundHandled);
                    }
                    (Some(child), VisitState::FoundNotHandled, &controller::Event::Up) => {
                        if let Some(next) = DisplayManager::prev_widget(root, child) {
                            manager.shift_focus(next);
                        } else {
                            manager.shift_focus(widget);
                        }
                        //TODO: if we can't take focus, propagate to our parent
                        return Ok(VisitState::FoundHandled);
                    }
                    _ => (),
                }
            }
            Ok(VisitState::NotFound)
        }

        visit(self, root, root, event, &mut actions)?;
        Ok(actions)
    }

    pub fn do_actions(
        &mut self,
        root: &mut Widget,
        mut actions: Vec<action::Action>,
    ) -> Result<()> {
        fn visit(
            manager: &mut DisplayManager,
            widget: &mut Widget,
            actions: &mut Vec<action::Action>,
        ) -> Result<()> {
            if actions.len() == 0 {
                return Ok(());
            }
            actions.retain(|action| match widget.do_action(manager, action) {
                Ok(handled) => !handled,
                Err(e) => {
                    println!(
                        "Error while processing '{:?}': {}",
                        action,
                        e.display_chain()
                    );
                    true
                }
            });

            for child in widget.mut_children() {
                visit(manager, child, actions)?;
            }

            Ok(())
        }

        visit(self, root, &mut actions)
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
