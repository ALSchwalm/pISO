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
    Relative(usize, usize),
    Normal,
}

pub struct Window {
    pub position: Position,
    pub size: (usize, usize),
    pub z: u32,
    pub focus: bool,
    pub id: WindowId,
    pub parent: WindowId,
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

    pub fn root(&self) -> WindowId {
        0
    }

    pub fn add_child(&mut self, parent: WindowId, pos: Position) -> Result<WindowId> {
        let id = self.nextid;
        self.nextid += 1;

        self.windows.insert(
            id,
            Window {
                position: pos,
                id: id,

                // Default the size to 0, 0 and set during the render to whatever
                // the size actually is
                size: (0, 0),

                //TODO: this should probably be the parent z + 1
                z: 0,
                focus: false,
                parent: parent,
            },
        );

        Ok(id)
    }

    pub fn remove_child(&mut self, id: WindowId) -> Result<()> {
        //TODO: remove children recursively
        self.windows.remove(&id);
        Ok(())
    }

    pub fn get(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }

    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    fn parent_widget<'a>(&'a self, root: &'a Widget, target: &'a Widget) -> Option<&'a Widget> {
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

    fn position_from_parent(&self, parent: &Widget, child: &Widget) -> (usize, usize) {
        let height = parent
            .children()
            .iter()
            .take_while(|widget| widget.windowid() != child.windowid())
            .filter_map(|widget| {
                let win = self.get(widget.windowid()).expect("widget has no window");
                match win.position {
                    Position::Normal => Some(win.size.1),
                    _ => None,
                }
            })
            .sum();

        (0, height)
    }

    fn calculate_position(&self, root: &Widget, widget: &Widget) -> (usize, usize) {
        let parent_widget = self.parent_widget(root, widget);
        let normal_offset = parent_widget
            .map(|parent| self.position_from_parent(parent, widget))
            .unwrap_or((0, 0));
        let parent_pos = parent_widget
            .map(|widget| self.calculate_position(root, widget))
            .unwrap_or((0, 0));

        let window = self.get(widget.windowid()).expect("widget has no window");

        match window.position {
            Position::Fixed(x, y) => (x, y),
            Position::Relative(x_off, y_off) => (
                parent_pos.0 + normal_offset.0 + x_off,
                parent_pos.1 + normal_offset.1 + y_off,
            ),
            Position::Normal => (
                parent_pos.0 + normal_offset.0,
                parent_pos.1 + normal_offset.1,
            ),
        }
    }

    fn first_descendant<'a>(&'a self, widget: &'a Widget) -> &'a Widget {
        widget
            .children()
            .first()
            .map(|child| self.first_descendant(*child))
            .unwrap_or(widget)
    }

    fn last_descendant<'a>(&'a self, widget: &'a Widget) -> &'a Widget {
        widget
            .children()
            .last()
            .map(|child| self.first_descendant(*child))
            .unwrap_or(widget)
    }

    fn next_widget<'a>(&'a self, root: &'a Widget, widget: &'a Widget) -> Option<&'a Widget> {
        if let Some(parent) = self.parent_widget(root, widget) {
            let children = parent.children();
            children
                .iter()
                .position(|child| widget.windowid() == child.windowid())
                .and_then(|pos| children.into_iter().nth(pos + 1))
                .map(|widget| self.first_descendant(widget))
        } else {
            None
        }
    }

    fn prev_widget<'a>(&'a self, root: &'a Widget, widget: &'a Widget) -> Option<&'a Widget> {
        if let Some(parent) = self.parent_widget(root, widget) {
            let children = parent.children();
            children
                .iter()
                .position(|child| widget.windowid() == child.windowid())
                .and_then(|pos| children.into_iter().nth(pos - 1))
                .map(|widget| self.last_descendant(widget))
        } else {
            None
        }
    }

    pub fn shift_focus(&mut self, window: WindowId) {
        println!("Shifting focus to window id={}", window);
        for (&id, cand) in self.windows.iter_mut() {
            if id == window {
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
                    (Some(widget), VisitState::FoundNotHandled, &controller::Event::Down) => {
                        let next_window_id = {
                            let next = manager.next_widget(root, widget);
                            next.map(|next| next.windowid())
                        };

                        //TODO: should we focus ourselves if there is not next window?
                        if let Some(next) = next_window_id {
                            manager.shift_focus(next);
                            return Ok(VisitState::FoundHandled);
                        } else {
                            println!("No 'next window' from this window id={}", widget.windowid());
                        }
                    }
                    (Some(widget), VisitState::FoundNotHandled, &controller::Event::Up) => {
                        let prev_window_id = {
                            let prev = manager.prev_widget(root, widget);
                            prev.map(|prev| prev.windowid())
                        };

                        if let Some(prev) = prev_window_id {
                            manager.shift_focus(prev);
                            return Ok(VisitState::FoundHandled);
                        } else {
                            println!("No 'prev window' from this window id={}", widget.windowid());
                        }
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

    pub fn render(&mut self, root: &Widget) -> Result<()> {
        fn render_window(
            manager: &mut DisplayManager,
            base: &mut bitmap::Bitmap,
            root: &Widget,
            widget: &Widget,
        ) -> Result<()> {
            println!("Rendering windowid={}", widget.windowid());
            //TODO: make this less terrible
            let pos = manager.calculate_position(root, widget);
            let bmap = {
                let mut window = manager
                    .get_mut(widget.windowid())
                    .ok_or(format!("failed to find window id={}", widget.windowid()))?;
                let bmap = widget.render(window)?;
                window.size = (bmap.width(), bmap.height());
                println!("Window size is ({}, {})", bmap.width(), bmap.height());
                bmap
            };

            println!("Blitting to ({}, {})", pos.0, pos.1);
            base.blit(bmap, pos);

            for child in widget.children() {
                render_window(manager, base, root, child)?
            }

            Ok(())
        };

        let mut bitmap = bitmap::Bitmap::new(0, 0);
        render_window(self, &mut bitmap, root, root)?;

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
