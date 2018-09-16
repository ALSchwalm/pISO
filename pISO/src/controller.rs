use config;
use error;
use mio;
use mio::*;
use std::time;
use sysfs_gpio::{AsyncPinPoller, Direction, Edge, Pin};

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Up,
    Down,
    Select,

    UpLong,
    DownLong,
    SelectLong,
}

#[allow(unused)]
pub struct Controller {
    config: config::Config,

    poll: Poll,
    events: <Events as IntoIterator>::IntoIter,
    up_input: Pin,
    down_input: Pin,
    select_input: Pin,

    up_started: Option<time::SystemTime>,
    up_poller: AsyncPinPoller,

    down_started: Option<time::SystemTime>,
    down_poller: AsyncPinPoller,

    select_started: Option<time::SystemTime>,
    select_poller: AsyncPinPoller,

    flipped: bool,
}

impl Controller {
    pub fn new(config: &config::Config) -> error::Result<Controller> {
        let up_input = Pin::new(27);
        up_input.export()?;
        up_input.set_direction(Direction::In)?;
        up_input.set_edge(Edge::BothEdges)?;
        let up_poller = up_input.get_async_poller()?;

        let down_input = Pin::new(22);
        down_input.export()?;
        down_input.set_direction(Direction::In)?;
        down_input.set_edge(Edge::BothEdges)?;
        let down_poller = down_input.get_async_poller()?;

        let select_input = Pin::new(17);
        select_input.export()?;
        select_input.set_direction(Direction::In)?;
        select_input.set_edge(Edge::BothEdges)?;
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
            up_started: None,
            up_poller: up_poller,
            down_started: None,
            down_poller: down_poller,
            select_started: None,
            select_poller: select_poller,
            flipped: false,
        })
    }

    pub fn flip_controls(&mut self) {
        self.flipped = !self.flipped;
    }

    fn event_value(&self, e: &mio::Event) -> u8 {
        match e.token() {
            Token(1) => self.up_input
                .get_value()
                .expect("Failed to get input value"),
            Token(2) => self.down_input
                .get_value()
                .expect("Failed to get input value"),
            Token(3) => self.select_input
                .get_value()
                .expect("Failed to get input value"),
            Token(_) => unreachable!(),
        }
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

            if event.readiness().is_readable() {
                let value = self.event_value(&event);
                let res = match event.token() {
                    Token(1) => {
                        if value == 0 {
                            self.up_started =
                                self.up_started.or_else(|| Some(time::SystemTime::now()));
                            println!("controller: UP pressed");
                            None
                        } else {
                            let pressed_duration = match self.up_started {
                                Some(t) => t.elapsed().expect("Failed to read system time"),
                                None => {
                                    println!("controller: UP released without press");
                                    continue;
                                }
                            };
                            self.up_started = None;

                            let nanos = pressed_duration.subsec_nanos() as u64;
                            let ms = (1000 * 1000 * 1000 * pressed_duration.as_secs() + nanos)
                                / (1000 * 1000);
                            println!("controller: UP released ({})", ms);

                            if pressed_duration > self.config.ui.button_long_press {
                                Some(Event::UpLong)
                            } else if pressed_duration > self.config.ui.min_button_press {
                                Some(Event::Up)
                            } else {
                                println!("controller: too short a press");
                                None
                            }
                        }
                    }
                    Token(2) => {
                        if value == 0 {
                            self.down_started =
                                self.down_started.or_else(|| Some(time::SystemTime::now()));
                            println!("controller: DOWN pressed");
                            None
                        } else {
                            let pressed_duration = match self.down_started {
                                Some(t) => t.elapsed().expect("Failed to read system time"),
                                None => {
                                    println!("controller: DOWN released without press");
                                    continue;
                                }
                            };
                            self.down_started = None;

                            let nanos = pressed_duration.subsec_nanos() as u64;
                            let ms = (1000 * 1000 * 1000 * pressed_duration.as_secs() + nanos)
                                / (1000 * 1000);
                            println!("controller: DOWN released ({})", ms);

                            if pressed_duration > self.config.ui.button_long_press {
                                Some(Event::DownLong)
                            } else if pressed_duration > self.config.ui.min_button_press {
                                Some(Event::Down)
                            } else {
                                println!("controller: too short a press");
                                None
                            }
                        }
                    }
                    Token(3) => {
                        if value == 0 {
                            self.select_started = self.select_started
                                .or_else(|| Some(time::SystemTime::now()));
                            println!("controller: SELECT pressed");
                            None
                        } else {
                            let pressed_duration = match self.select_started {
                                Some(t) => t.elapsed().expect("Failed to read system time"),
                                None => {
                                    println!("controller: SELECT released without press");
                                    continue;
                                }
                            };
                            self.select_started = None;

                            let nanos = pressed_duration.subsec_nanos() as u64;
                            let ms = (1000 * 1000 * 1000 * pressed_duration.as_secs() + nanos)
                                / (1000 * 1000);
                            println!("controller: SELECT released ({})", ms);

                            if pressed_duration > self.config.ui.button_long_press {
                                Some(Event::SelectLong)
                            } else if pressed_duration > self.config.ui.min_button_press {
                                Some(Event::Select)
                            } else {
                                println!("controller: too short a press");
                                None
                            }
                        }
                    }
                    Token(_) => unreachable!(),
                };

                match res {
                    Some(event) => {
                        return if self.flipped {
                            match event {
                                Event::Up => Some(Event::Down),
                                Event::UpLong => Some(Event::DownLong),
                                Event::Down => Some(Event::Up),
                                Event::DownLong => Some(Event::UpLong),
                                other => Some(other),
                            }
                        } else {
                            Some(event)
                        }
                    }
                    None => continue,
                }
            }
        }
    }
}
