use config;
use error;
use mio::*;
use std::thread;
use std::time;
use sysfs_gpio::{AsyncPinPoller, Direction, Edge, Pin};

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Up,
    Down,
    Select,
}

#[allow(unused)]
pub struct Controller {
    config: config::Config,

    poll: Poll,
    events: <Events as IntoIterator>::IntoIter,
    up_input: Pin,
    down_input: Pin,
    select_input: Pin,
    up_poller: AsyncPinPoller,
    down_poller: AsyncPinPoller,
    select_poller: AsyncPinPoller,
    last_event: time::SystemTime,
}

impl Controller {
    pub fn new(config: &config::Config) -> error::Result<Controller> {
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
            config: config.clone(),
            poll: poll,
            events: events.into_iter(),
            up_input: up_input,
            down_input: down_input,
            select_input: select_input,
            up_poller: up_poller,
            down_poller: down_poller,
            select_poller: select_poller,
            last_event: time::SystemTime::now(),
        })
    }
}

impl Iterator for Controller {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let event = match self.events.next() {
                Some(event) => event,
                None => {
                    let mut events = Events::with_capacity(1024);
                    self.poll
                        .poll(&mut events, None)
                        .expect("Failed to poll inputs");
                    self.events = events.into_iter();
                    if let Some(event) = self.events.next() {
                        event
                    } else {
                        continue;
                    }
                }
            };

            if self.last_event.elapsed().unwrap() > self.config.ui.debounce_delay
                && event.readiness().is_readable()
            {
                thread::sleep(self.config.ui.debounce_min_hold);

                let res = match event.token() {
                    Token(1) => {
                        if self.up_input
                            .get_value()
                            .expect("Failed to get input value") != 0
                        {
                            continue;
                        }
                        Some(Event::Up)
                    }
                    Token(2) => {
                        if self.down_input
                            .get_value()
                            .expect("Failed to get input value") != 0
                        {
                            continue;
                        }
                        Some(Event::Down)
                    }
                    Token(3) => {
                        if self.select_input
                            .get_value()
                            .expect("Failed to get input value") != 0
                        {
                            continue;
                        }
                        Some(Event::Select)
                    }
                    Token(_) => unreachable!(),
                };
                self.last_event = time::SystemTime::now();
                return res;
            }
        }
    }
}
