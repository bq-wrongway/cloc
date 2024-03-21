use iced::advanced::Application;
use iced::alignment::Horizontal;
use iced::alignment::Vertical;
use iced::executor;
use iced::mouse;
use iced::widget::button;
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::slider;
use iced::widget::{canvas, container};
use iced::widget::{column, Row};
use iced::Color;
use iced::Event;
use iced::Point;
use iced::Radians;
use iced::Size;
use iced::{Command, Element, Length, Rectangle, Renderer, Settings, Subscription, Theme};
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
            resizable: true,
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
    is_settings_open: bool,
    color_r: u16,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(time::OffsetDateTime),
    MouseClicked,
    RightClick,
    MessageColor(u16),
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
                is_settings_open: false,
                color_r: 125,
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
            Message::RightClick => {
                self.is_settings_open = !self.is_settings_open;
                iced::window::resize(
                    iced::window::Id::MAIN,
                    Size {
                        height: 400.0,
                        width: 800.0,
                    },
                )
                // Command::none()
            }
            Message::MessageColor(c) => {
                self.color_r = c;
                println!("{:?}", c);
                self.seconds_color =
                    Color::from_rgba(c as f32 / 255.0, 40.0 / 255.0, 145.0 / 255.0, 0.4);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let clock = canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill);

        let cc = container(clock)
            // .style(container::bordered_box)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Top)
            .width(Length::FillPortion(2))
            .height(Length::Fill);
        let main_container = Row::new().width(Length::Fill).height(Length::Fill).push(cc);
        if self.is_settings_open {
            let h_slider =
                container(slider(0..=255, self.color_r, Message::MessageColor)).width(250);
            main_container
                .push(
                    container(column![button("text"), h_slider])
                        .width(Length::FillPortion(3))
                        .height(Length::Fill)
                        .style(container::rounded_box)
                        .align_y(Vertical::Center)
                        .align_x(Horizontal::Center),
                )
                .into()
        } else {
            main_container.into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::event::listen_with(|event, _status| match event {
                Event::Mouse(e) => match e {
                    mouse::Event::ButtonPressed(mouse::Button::Middle) => {
                        Some(Message::MouseClicked)
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Right) => Some(Message::RightClick),
                    _ => None,
                },
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
                ..Theme::Dark.palette()
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
            let offset_angle = Radians(PI * 1.5);
            builder.arc(canvas::path::Arc {
                center: frame.center(),
                radius: radius - bar_height,
                start_angle: offset_angle,
                end_angle: Radians(circle_rotation(self.now.second(), 60)),
            });
            builder2.arc(canvas::path::Arc {
                center: frame.center(),
                radius: radius - bar_height * 2.0,
                start_angle: offset_angle,
                end_angle: Radians(circle_rotation(self.now.minute(), 60)),
            });
            builder3.arc(canvas::path::Arc {
                center: frame.center(),
                radius: radius - (bar_height * 2.0) - bar_height,
                start_angle: offset_angle,
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
                size: ((radius / 3.0) * 0.4).into(),
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
