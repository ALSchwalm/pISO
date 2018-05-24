use action;
use bitmap;
use controller;
use display;
use error::{Result, ResultExt};
use error_chain::ChainedError;
use input;
use render;
use std::collections::BTreeMap;

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
    pub display: Box<display::Display>,
    windows: BTreeMap<WindowId, Window>,
    nextid: u32,
}

impl DisplayManager {
    pub fn new(mut disp: Box<display::Display>) -> Result<DisplayManager> {
        disp.on().chain_err(|| "Failed to activate display")?;

        Ok(DisplayManager {
            display: disp,
            windows: BTreeMap::new(),
            nextid: 1,
        })
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

    pub fn get(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }

    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    fn nearest_fixed_position_ancestor<'a, 'b>(
        &'a self,
        root: &'b Widget,
        target: &'b Widget,
    ) -> Result<&'b Widget> {
        fn visit<'a, 'b>(
            disp: &'a DisplayManager,
            current: &'b Widget,
            target: &'b Widget,
        ) -> (bool, Option<&'b Widget>) {
            if current.windowid() == target.windowid() {
                (true, None)
            } else {
                let window = disp.get(current.windowid()).expect("widget has no window");

                for child in current.children() {
                    let res = visit(disp, child, target);
                    if res.0 {
                        if res.1.is_some() {
                            return res;
                        }

                        match window.position {
                            Position::Normal => return res,
                            Position::Fixed(_, _) => return (true, Some(current)),
                        }
                    }
                }
                (false, None)
            }
        }

        let window = self.get(target.windowid()).expect("widget has no window");
        match window.position {
            Position::Normal => visit(self, root, target)
                .1
                .ok_or("Failed to find fixed position parent".into()),
            Position::Fixed(_, _) => Ok(target),
        }
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
                let window = disp.get(current.windowid()).expect("widget has no window");
                let height = match window.position {
                    Position::Fixed(_, _) => 0,
                    Position::Normal => window.bitmap.height(),
                };
                for child in current.children() {
                    // Skip other fixed position windows
                    let window = disp.get(child.windowid()).expect("widget has no window");
                    match window.position {
                        Position::Fixed(_, _) => continue,
                        _ => (),
                    };

                    let res = visit(disp, child, target, pos);
                    if res.0 {
                        return res;
                    } else {
                        pos = (res.1, res.2);
                    }
                }
                pos = (pos.0, pos.1 + height);
                (false, pos.0, pos.1)
            }
        }
        let parent = self.nearest_fixed_position_ancestor(root, widget)
            .expect("Widget has no fixed position parent");
        let parent_window = self.get(parent.windowid()).expect("widget has no window");
        let parent_offset = match parent_window.position {
            Position::Fixed(x, y) => (x, y),
            Position::Normal => panic!("nearest fixed position parent is Normal position"),
        };

        let res = visit(self, parent, widget, parent_offset);
        if !res.0 {
            panic!("Unable to find widget with id={}", widget.windowid());
        } else {
            (res.1, res.2)
        }
    }

    pub fn calculate_position(&self, root: &Widget, widget: &Widget) -> (usize, usize) {
        let window = self.get(widget.windowid()).expect("widget has no window");

        match window.position {
            Position::Fixed(x, y) => (x, y),
            Position::Normal => self.normal_position(root, widget),
        }
    }

    fn is_fixed_position(&self, widget: &Widget) -> bool {
        match self.get(widget.windowid())
            .expect("widget has no window")
            .position
        {
            Position::Fixed(_, _) => true,
            _ => false,
        }
    }

    fn nearest_widget<'a, 'b>(
        &'a self,
        root: &'b Widget,
        target: &'b Widget,
        next: bool,
    ) -> Option<&'b Widget> {
        fn visit<'a, 'b>(
            disp: &'a DisplayManager,
            current: &'b Widget,
            target: &'b Widget,
            next: bool,
        ) -> (bool, Option<&'b Widget>) {
            if current.windowid() == target.windowid() {
                return (true, current.children().first().map(|n| *n));
            } else {
                let children = if next {
                    current.children()
                } else {
                    current
                        .children()
                        .iter()
                        .rev()
                        .map(|c| *c)
                        .collect::<Vec<_>>()
                };
                for (idx, child) in children.iter().enumerate() {
                    if disp.is_fixed_position(*child) {
                        continue;
                    }
                    let res = visit(disp, *child, target, next);
                    if res.0 {
                        if res.1.is_none() {
                            return (
                                true,
                                children
                                    .iter()
                                    .skip(idx + 1)
                                    .filter(|child| !disp.is_fixed_position(**child))
                                    .next()
                                    .map(|n| *n),
                            );
                        } else {
                            return res;
                        }
                    }
                }
                return (false, None);
            }
        }

        let parent = self.nearest_fixed_position_ancestor(root, target)
            .expect("Widget has no fixed position parent");
        visit(self, parent, target, next).1
    }

    fn next_widget<'a, 'b>(&'a self, root: &'b Widget, target: &'b Widget) -> Option<&'b Widget> {
        self.nearest_widget(root, target, true)
    }

    fn prev_widget<'a, 'b>(&'a self, root: &'b Widget, target: &'b Widget) -> Option<&'b Widget> {
        self.nearest_widget(root, target, false)
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

    fn find_focused_widget_mut<'a>(&self, current: &'a mut Widget) -> Option<&'a mut Widget> {
        let focus = self.get(current.windowid())
            .expect("Unable to find focused window")
            .focus;
        if focus {
            return Some(current);
        }
        for child in current.mut_children().into_iter() {
            if let Some(child) = self.find_focused_widget_mut(child) {
                return Some(child);
            }
        }
        None
    }

    fn find_focused_widget<'a>(&self, current: &'a Widget) -> Option<&'a Widget> {
        let focus = self.get(current.windowid())
            .expect("Unable to find focused window")
            .focus;
        if focus {
            return Some(current);
        }
        for child in current.children().into_iter() {
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
        {
            let focused = self.find_focused_widget_mut(root)
                .expect("No focused window");
            let event_res = focused.on_event(event)?;
            if event_res.0 {
                return Ok(event_res.1);
            }
        }
        let focused = self.find_focused_widget(root).expect("No focused window");

        match event {
            &controller::Event::Up => {
                if let Some(prev) = self.prev_widget(root, focused) {
                    // Special case, do not focus the root
                    if prev.windowid() != root.windowid() {
                        self.shift_focus(prev);
                    }
                } else {
                    println!("No previous widget for window id={}", focused.windowid());
                }
            }
            &controller::Event::Down => {
                if let Some(next) = self.next_widget(root, focused) {
                    self.shift_focus(next);
                } else {
                    println!("No next widget for window id={}", focused.windowid());
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
    pub fn do_render(&mut self, root: &Widget) -> Result<()> {
        fn visit(manager: &mut DisplayManager, widget: &Widget) -> Result<()> {
            let bitmap = {
                let window = manager
                    .get(widget.windowid())
                    .ok_or(format!("failed to find window id={}", widget.windowid()))?;
                widget.render(manager, window)?
            };
            {
                let window = manager
                    .get_mut(widget.windowid())
                    .ok_or(format!("failed to find window id={}", widget.windowid()))?;
                window.bitmap = bitmap;
            }

            // Render from the bottom up
            for child in widget.children() {
                visit(manager, child)?;
            }

            Ok(())
        }

        visit(self, root)
    }

    fn do_blit(
        &mut self,
        real_root: &Widget,
        root: &Widget,
        bitmap: &mut bitmap::Bitmap,
    ) -> Result<()> {
        fn position_window<'a>(
            manager: &DisplayManager,
            base: &mut bitmap::Bitmap,
            real_root: &Widget,
            root: &Widget,
            widget: &'a Widget,
            fixed_pos_windows: &mut Vec<&'a Widget>,
            scroll_shift: i32,
        ) -> Result<()> {
            println!("Positioning windowid={}", widget.windowid());

            let pos = manager.calculate_position(real_root, widget);
            let window = manager
                .get(widget.windowid())
                .ok_or(format!("failed to find window id={}", widget.windowid()))?;

            println!(
                "Blitting to ({}, {}, size=({}, {}) scroll_shift={})",
                pos.0,
                pos.1,
                window.bitmap.width(),
                window.bitmap.height(),
                scroll_shift
            );

            base.blit_clip(&window.bitmap, (pos.0 as i32, pos.1 as i32 - scroll_shift));
            for child in widget.children() {
                let window = manager
                    .get(child.windowid())
                    .ok_or(format!("failed to find window id={}", child.windowid()))?;
                match window.position {
                    Position::Fixed(_, _) => {
                        fixed_pos_windows.push(child);
                    }
                    _ => {
                        position_window(
                            manager,
                            base,
                            real_root,
                            root,
                            child,
                            fixed_pos_windows,
                            scroll_shift,
                        )?;
                    }
                };
            }

            Ok(())
        };

        let mut fixed_pos_windows = vec![];
        let scroll_shift = self.find_scroll_shift(real_root, root).unwrap_or(0);
        position_window(
            self,
            bitmap,
            real_root,
            root,
            root,
            &mut fixed_pos_windows,
            scroll_shift,
        )?;

        for fixed in fixed_pos_windows {
            self.do_blit(real_root, fixed, bitmap)?;
        }

        Ok(())
    }

    fn find_scroll_shift(&self, root: &Widget, widget: &Widget) -> Option<i32> {
        for child in widget.children() {
            let window = self.get(child.windowid()).unwrap();
            match window.position {
                Position::Fixed(_, _) => (),
                Position::Normal => {
                    if window.focus {
                        let pos = self.calculate_position(root, child);
                        let bottom = pos.1 as i32 + window.bitmap.height() as i32;
                        if bottom > display::DISPLAY_HEIGHT as i32 {
                            return Some(bottom - display::DISPLAY_HEIGHT as i32);
                        } else {
                            return None;
                        }
                    } else {
                        if let Some(shift) = self.find_scroll_shift(root, child) {
                            return Some(shift);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn render(&mut self, root: &Widget) -> Result<()> {
        self.do_render(root)?;

        let mut bitmap = bitmap::Bitmap::new(display::DISPLAY_WIDTH, display::DISPLAY_HEIGHT);
        self.do_blit(root, root, &mut bitmap)?;
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
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        vec![]
    }
    fn children(&self) -> Vec<&Widget> {
        vec![]
    }
    fn windowid(&self) -> WindowId;
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone)]
    struct TestWidget {
        pub windowid: WindowId,
        pub size: (usize, usize),
        pub children: Vec<TestWidget>,
    }

    impl render::Render for TestWidget {
        fn render(&self, _manager: &DisplayManager, _window: &Window) -> Result<bitmap::Bitmap> {
            Ok(bitmap::Bitmap::new(self.size.0, self.size.1))
        }
    }

    impl input::Input for TestWidget {
        fn on_event(&mut self, _event: &controller::Event) -> Result<(bool, Vec<action::Action>)> {
            Ok((false, vec![]))
        }
    }

    impl Widget for TestWidget {
        fn mut_children(&mut self) -> Vec<&mut Widget> {
            self.children
                .iter_mut()
                .map(|widget| widget as &mut Widget)
                .collect()
        }
        fn children(&self) -> Vec<&Widget> {
            self.children
                .iter()
                .map(|widget| widget as &Widget)
                .collect()
        }
        fn windowid(&self) -> WindowId {
            self.windowid
        }
    }

    impl TestWidget {
        fn new(
            disp: &mut DisplayManager,
            position: Position,
            size: (usize, usize),
            children: Vec<TestWidget>,
        ) -> Result<TestWidget> {
            Ok(TestWidget {
                windowid: disp.add_child(position)?,
                size: size,
                children: children,
            })
        }
    }

    #[test]
    // Test that a bunch of normally positioned windows just line up vertically
    fn test_calc_basic_position() {
        let display = Box::new(display::test::TestDisplay {});
        let mut manager = DisplayManager::new(display).expect("Failed to create displaymanager");

        let child1 = TestWidget::new(&mut manager, Position::Normal, (0, 10), vec![])
            .expect("Failed to create test widget");
        let child2 = TestWidget::new(&mut manager, Position::Normal, (0, 10), vec![])
            .expect("Failed to create test widget");
        let root = TestWidget::new(
            &mut manager,
            Position::Fixed(0, 0),
            (0, 0),
            vec![child1.clone(), child2.clone()],
        ).expect("Failed to create test widget");

        manager.do_render(&root).expect("Failed to do_render");
        assert_eq!(manager.calculate_position(&root, &root), (0, 0));
        assert_eq!(manager.calculate_position(&root, &child1), (0, 0));
        assert_eq!(manager.calculate_position(&root, &child2), (0, 10));
    }

    #[test]
    // Test that focus moves up and down normal positioned windows as expected
    fn test_basic_focus() {
        let display = Box::new(display::test::TestDisplay {});
        let mut manager = DisplayManager::new(display).expect("Failed to create displaymanager");

        let child1 = TestWidget::new(&mut manager, Position::Normal, (0, 0), vec![])
            .expect("Failed to create test widget");
        let child2 = TestWidget::new(&mut manager, Position::Normal, (0, 0), vec![])
            .expect("Failed to create test widget");
        let mut root = TestWidget::new(
            &mut manager,
            Position::Fixed(0, 0),
            (0, 0),
            vec![child1.clone(), child2.clone()],
        ).expect("Failed to create test widget");

        manager.shift_focus(&child1);
        assert_eq!(
            manager.find_focused_widget(&mut root).unwrap().windowid(),
            child1.windowid()
        );
        manager.on_event(&mut root, &controller::Event::Up).unwrap();
        assert_eq!(
            manager.find_focused_widget(&mut root).unwrap().windowid(),
            child1.windowid()
        );
        manager
            .on_event(&mut root, &controller::Event::Down)
            .unwrap();
        assert_eq!(
            manager.find_focused_widget(&mut root).unwrap().windowid(),
            child2.windowid()
        );
        manager.on_event(&mut root, &controller::Event::Up).unwrap();
        assert_eq!(
            manager.find_focused_widget(&mut root).unwrap().windowid(),
            child1.windowid()
        );
    }

    #[test]
    // Test that fixed position windows are ignored in the flow of normal positioned windows
    fn test_calc_normal_with_fixed_position() {
        let display = Box::new(display::test::TestDisplay {});
        let mut manager = DisplayManager::new(display).expect("Failed to create displaymanager");

        let child1 = TestWidget::new(&mut manager, Position::Normal, (0, 10), vec![])
            .expect("Failed to create test widget");
        let child2 = TestWidget::new(&mut manager, Position::Fixed(20, 20), (50, 50), vec![])
            .expect("Failed to create test widget");
        let child3 = TestWidget::new(&mut manager, Position::Normal, (0, 10), vec![])
            .expect("Failed to create test widget");
        let root = TestWidget::new(
            &mut manager,
            Position::Fixed(0, 0),
            (0, 0),
            vec![child1.clone(), child2.clone(), child3.clone()],
        ).expect("Failed to create test widget");

        manager.do_render(&root).expect("Failed to do_render");
        assert_eq!(manager.calculate_position(&root, &root), (0, 0));
        assert_eq!(manager.calculate_position(&root, &child1), (0, 0));
        assert_eq!(manager.calculate_position(&root, &child2), (20, 20));
        assert_eq!(manager.calculate_position(&root, &child3), (0, 10));
    }

    #[test]
    // Test that fixed position windows are ignored when shifting focus
    fn test_normal_focus_with_fixed_position() {
        let display = Box::new(display::test::TestDisplay {});
        let mut manager = DisplayManager::new(display).expect("Failed to create displaymanager");

        let child1 = TestWidget::new(&mut manager, Position::Normal, (0, 0), vec![])
            .expect("Failed to create test widget");
        let child2 = TestWidget::new(&mut manager, Position::Fixed(20, 20), (0, 0), vec![])
            .expect("Failed to create test widget");
        let child3 = TestWidget::new(&mut manager, Position::Normal, (0, 0), vec![])
            .expect("Failed to create test widget");
        let mut root = TestWidget::new(
            &mut manager,
            Position::Fixed(0, 0),
            (0, 0),
            vec![child1.clone(), child2.clone(), child3.clone()],
        ).expect("Failed to create test widget");

        manager.shift_focus(&child1);
        manager
            .on_event(&mut root, &controller::Event::Down)
            .unwrap();
        assert_eq!(
            manager.find_focused_widget(&mut root).unwrap().windowid(),
            child3.windowid()
        );
        manager.on_event(&mut root, &controller::Event::Up).unwrap();
        assert_eq!(
            manager.find_focused_widget(&mut root).unwrap().windowid(),
            child1.windowid()
        );
    }

    #[test]
    // Test that windows are aligned vertically with their nearest fixed position parent
    fn test_calc_normal_within_fixed() {
        let display = Box::new(display::test::TestDisplay {});
        let mut manager = DisplayManager::new(display).expect("Failed to create displaymanager");

        let child1 = TestWidget::new(&mut manager, Position::Normal, (0, 10), vec![])
            .expect("Failed to create test widget");

        let child2_1 = TestWidget::new(&mut manager, Position::Normal, (50, 50), vec![])
            .expect("Failed to create test widget");
        let child2 = TestWidget::new(
            &mut manager,
            Position::Fixed(20, 20),
            (100, 100),
            vec![child2_1.clone()],
        ).expect("Failed to create test widget");

        let child3 = TestWidget::new(&mut manager, Position::Normal, (0, 10), vec![])
            .expect("Failed to create test widget");
        let root = TestWidget::new(
            &mut manager,
            Position::Fixed(0, 0),
            (0, 0),
            vec![child1.clone(), child2.clone(), child3.clone()],
        ).expect("Failed to create test widget");

        manager.do_render(&root).expect("Failed to do_render");
        assert_eq!(manager.calculate_position(&root, &root), (0, 0));
        assert_eq!(manager.calculate_position(&root, &child1), (0, 0));
        assert_eq!(manager.calculate_position(&root, &child2), (20, 20));
        assert_eq!(manager.calculate_position(&root, &child2_1), (20, 20));
        assert_eq!(manager.calculate_position(&root, &child3), (0, 10));
    }

    #[test]
    // Test that focus moves within the nearest fixed position parent
    fn test_normal_within_fixed_focus() {
        let display = Box::new(display::test::TestDisplay {});
        let mut manager = DisplayManager::new(display).expect("Failed to create displaymanager");

        let child1 = TestWidget::new(&mut manager, Position::Normal, (0, 0), vec![])
            .expect("Failed to create test widget");

        let child2_1 = TestWidget::new(&mut manager, Position::Normal, (0, 0), vec![])
            .expect("Failed to create test widget");
        let child2_2 = TestWidget::new(&mut manager, Position::Normal, (0, 0), vec![])
            .expect("Failed to create test widget");
        let child2 = TestWidget::new(
            &mut manager,
            Position::Fixed(20, 20),
            (0, 0),
            vec![child2_1.clone(), child2_2.clone()],
        ).expect("Failed to create test widget");

        let child3 = TestWidget::new(&mut manager, Position::Normal, (0, 0), vec![])
            .expect("Failed to create test widget");
        let mut root = TestWidget::new(
            &mut manager,
            Position::Fixed(0, 0),
            (0, 0),
            vec![child1.clone(), child2.clone(), child3.clone()],
        ).expect("Failed to create test widget");

        manager.shift_focus(&child2_1);
        manager
            .on_event(&mut root, &controller::Event::Down)
            .unwrap();
        assert_eq!(
            manager.find_focused_widget(&mut root).unwrap().windowid(),
            child2_2.windowid()
        );
        manager
            .on_event(&mut root, &controller::Event::Down)
            .unwrap();
        assert_eq!(
            manager.find_focused_widget(&mut root).unwrap().windowid(),
            child2_2.windowid()
        );
    }
}
