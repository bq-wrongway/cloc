use iced::alignment::Horizontal;
use iced::alignment::Vertical;
use iced::executor;
use iced::mouse;
use iced::theme::Container;
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::{canvas, container};
use iced::Color;
use iced::Event;
use iced::Point;
use iced::Radians;
use iced::Size;
use iced::{
    Application, Command, Element, Length, Rectangle, Renderer, Settings, Subscription, Theme,
};
use std::f32::consts::PI;
use time::format_description;
use time::macros::offset;
use time::UtcOffset;

pub fn main() -> iced::Result {
    Clock::run(Settings {
        antialiasing: true,
        window: iced::window::Settings {
            transparent: true,
            decorations: false,
            resizable: false,
            size: Size {
                width: 400.0,
                height: 400.0,
            },
            ..Default::default()
        },
        ..Settings::default()
    })
}

struct Clock {
    now: time::OffsetDateTime,
    clock: Cache,
    seconds_color: Color,
    minutes_color: Color,
    hours_color: Color,
    text_color: Color,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(time::OffsetDateTime),
    MouseClicked,
}

impl Application for Clock {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Clock {
                now: time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc().to_offset(offset!(+1))),
                clock: Cache::default(),
                hours_color: Color::WHITE,
                minutes_color: Color::TRANSPARENT,
                seconds_color: Color::from_rgba(245.0 / 255.0, 40.0 / 255.0, 145.0 / 255.0, 0.4),
                text_color: Color::WHITE,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Clock - Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                let now = local_time;
                if now != self.now {
                    self.now = now;
                    self.clock.clear();
                }
                Command::none()
            }

            Message::MouseClicked => iced::window::drag(iced::window::Id::MAIN),
        }
    }

    fn view(&self) -> Element<Message> {
        let canvas = canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill);

        container(canvas)
            .style(Container::Transparent)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::event::listen_with(|event, _status| match event {
                Event::Mouse(e) => {
                    if let mouse::Event::ButtonPressed(mouse::Button::Left) = e {
                        Some(Message::MouseClicked)
                    } else {
                        None
                    }
                }
                _ => None,
            }),
            iced::time::every(std::time::Duration::from_millis(500)).map(|_| {
                Message::Tick(time::OffsetDateTime::now_local().unwrap_or_else(|_| {
                    time::OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(1, 0, 0).unwrap())
                }))
            }),
        ])
    }

    fn theme(&self) -> Self::Theme {
        Theme::custom(
            "Custom".to_string(),
            iced::theme::Palette {
                background: Color::TRANSPARENT,
                ..Theme::Dracula.palette()
            },
        )
    }
}

impl<Message> canvas::Program<Message> for Clock {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let clock = self.clock.draw(renderer, bounds.size(), |frame| {
            let palette = theme.extended_palette();

            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;
            let bar_height = radius * 0.15;
            let second_path = Path::circle(center, radius - bar_height);
            let hour_path = Path::circle(center, radius - bar_height * 2.0);
            let minute_path = Path::circle(center, radius - (bar_height * 2.0) - bar_height);

            frame.stroke(
                &second_path,
                canvas::Stroke::default()
                    .with_color(self.seconds_color)
                    .with_width(bar_height),
            );
            frame.stroke(
                &hour_path,
                canvas::Stroke::default()
                    .with_color(self.hours_color)
                    .with_width(bar_height),
            );
            frame.stroke(
                &minute_path,
                canvas::Stroke::default()
                    .with_color(self.minutes_color)
                    .with_width(bar_height),
            );

            let mut builder = canvas::path::Builder::new();
            let mut builder2 = canvas::path::Builder::new();
            let mut builder3 = canvas::path::Builder::new();
            builder.arc(canvas::path::Arc {
                center: frame.center(),
                radius: radius - bar_height,
                start_angle: Radians(PI * 1.5),
                end_angle: Radians(circle_rotation(self.now.second(), 60)),
            });
            builder2.arc(canvas::path::Arc {
                center: frame.center(),
                radius: radius - bar_height * 2.0,
                start_angle: Radians(PI * 1.5),
                end_angle: Radians(circle_rotation(self.now.minute(), 60)),
            });
            builder3.arc(canvas::path::Arc {
                center: frame.center(),
                radius: radius - (bar_height * 2.0) - bar_height,
                start_angle: Radians(PI * 1.5),
                end_angle: Radians(circle_rotation(get_hr(self.now.hour()), 12)),
            });
            let (hrs, min, _sec) = self.now.time().as_hms();

            let text = canvas::Text {
                content: format!["{:02}:{:02}", hrs, min,],
                size: (radius / 3.0).into(),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                position: center,
                color: self.text_color,
                ..Default::default()
            };
            let date = self.now.date();
            let format = format_description::parse("[day]-[month]-[year]").unwrap();
            let date_text = canvas::Text {
                content: date.format(&format).unwrap(),
                color: Color::WHITE,
                size: text.size * 0.4,
                position: Point::new(frame.center().x, frame.center().y + radius / 4.5),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                ..Default::default()
            };
            let minutes_path_bd = builder2.build();
            let seconds_path_bd = builder.build();
            let hours_path_bd = builder3.build();
            let mut clr = self.seconds_color;
            clr.a = 1.0;
            frame.fill_text(text);
            frame.fill_text(date_text);
            frame.stroke(
                &seconds_path_bd,
                canvas::Stroke::default()
                    .with_color(clr)
                    .with_width(bar_height),
            );
            frame.stroke(
                &minutes_path_bd,
                canvas::Stroke::default()
                    .with_color(palette.success.strong.color)
                    .with_width(bar_height),
            );
            frame.stroke(
                &hours_path_bd,
                canvas::Stroke::default()
                    .with_color(palette.primary.base.color)
                    .with_width(bar_height),
            );
        });

        vec![clock]
    }
}

// fn hand_rotation(n: u8, total: u8) -> f32 {
//     let turns = n as f32 / total as f32;

//     2.0 * std::f32::consts::PI * turns
// }
fn circle_rotation(n: u8, total: u8) -> f32 {
    let turns = n as f32 / total as f32;

    PI * 1.5 + (2.0 * std::f32::consts::PI * turns)
}

fn get_hr(n: u8) -> u8 {
    // let mut hr: u8 = 0;
    if n == 0 {
        0
    } else if n >= 12 {
        n - 12
    } else {
        n
    }
}
