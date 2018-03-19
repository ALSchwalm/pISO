use error;
use mio::*;
use std::thread;
use std::time;
use sysfs_gpio::{AsyncPinPoller, Direction, Edge, Pin};

pub struct Controller {
    poll: Poll,
    events: Events,
    on_select_callback: Option<Box<FnMut()>>,
    on_up_callback: Option<Box<FnMut()>>,
    on_down_callback: Option<Box<FnMut()>>,
    up_input: Pin,
    down_input: Pin,
    select_input: Pin,
    up_poller: AsyncPinPoller,
    down_poller: AsyncPinPoller,
    select_poller: AsyncPinPoller,
}

impl Controller {
    pub fn new() -> error::Result<Controller> {
        let up_input = Pin::new(27);
        up_input.export()?;
        up_input.set_direction(Direction::In)?;
        up_input.set_edge(Edge::FallingEdge)?;
        let up_poller = up_input.get_async_poller()?;

        let down_input = Pin::new(22);
        down_input.export()?;
        down_input.set_direction(Direction::In)?;
        down_input.set_edge(Edge::FallingEdge)?;
        let down_poller = down_input.get_async_poller()?;

        let select_input = Pin::new(17);
        select_input.export()?;
        select_input.set_direction(Direction::In)?;
        select_input.set_edge(Edge::FallingEdge)?;
        let select_poller = select_input.get_async_poller().unwrap();

        let events = Events::with_capacity(1024);
        let poll = Poll::new().unwrap();

        poll.register(&up_poller, Token(1), Ready::readable(), PollOpt::edge())?;
        poll.register(&down_poller, Token(2), Ready::readable(), PollOpt::edge())?;
        poll.register(&select_poller, Token(3), Ready::readable(), PollOpt::edge())?;

        Ok(Controller {
            poll: poll,
            events: events,
            on_select_callback: None,
            on_up_callback: None,
            on_down_callback: None,
            up_input: up_input,
            down_input: down_input,
            select_input: select_input,
            up_poller: up_poller,
            down_poller: down_poller,
            select_poller: select_poller,
        })
    }

    pub fn on_select(&mut self, callback: Box<FnMut()>) {
        self.on_select_callback = Some(callback);
    }

    pub fn on_up(&mut self, callback: Box<FnMut()>) {
        self.on_up_callback = Some(callback);
    }

    pub fn on_down(&mut self, callback: Box<FnMut()>) {
        self.on_down_callback = Some(callback);
    }

    pub fn start(mut self) -> error::Result<()> {
        let debounce_delay = time::Duration::from_millis(2);
        let debounce_min_hold = time::Duration::from_millis(1);
        let mut last_event = time::SystemTime::now();

        loop {
            self.poll.poll(&mut self.events, None)?;

            for e in self.events.iter() {
                if last_event.elapsed().unwrap() > debounce_delay && e.readiness().is_readable() {
                    thread::sleep(debounce_min_hold);

                    match e.token() {
                        Token(1) => {
                            if self.up_input.get_value()? != 0 {
                                continue;
                            }
                            if let Some(ref mut callback) = self.on_up_callback {
                                (callback)();
                            }
                        }
                        Token(2) => {
                            if self.down_input.get_value()? != 0 {
                                continue;
                            }
                            if let Some(ref mut callback) = self.on_down_callback {
                                (callback)();
                            }
                        }
                        Token(3) => {
                            if self.select_input.get_value()? != 0 {
                                continue;
                            }
                            if let Some(ref mut callback) = self.on_select_callback {
                                (callback)();
                            }
                        }
                        Token(_) => unreachable!(),
                    }
                    last_event = time::SystemTime::now();
                }
            }
        }
    }
}
