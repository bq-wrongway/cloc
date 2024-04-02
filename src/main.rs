use iced::advanced::Application;
use iced::alignment::Horizontal;
use iced::alignment::Vertical;
use iced::executor;
use iced::mouse;
use iced::widget::button;
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::row;
use iced::widget::slider;
use iced::widget::text;
use iced::widget::{canvas, container};
use iced::widget::{column, Row};
use iced::Border;
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
    color_r: u8,
    color_g: u8,
    color_b: u8,
    opacity: f32,
    temp_color: Color,
    size: Size,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(time::OffsetDateTime),
    MouseClicked,
    RightClick,
    ColorRed(u8),
    ColorGreen(u8),
    ColorBlue(u8),
    Opacity(f32),
    Seconds,
    Minutes,
    Hours,
    Resize,
    Width(f32),
}

impl Application for Clock {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Renderer = Renderer;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Clock {
                now: time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc().to_offset(offset!(+1))),
                clock: Cache::default(),
                hours_color: Color::from_rgba(45.0 / 255.0, 140.0 / 255.0, 5.0 / 255.0, 0.4),
                minutes_color: Color::from_rgba(45.0 / 255.0, 40.0 / 255.0, 245.0 / 255.0, 0.4),
                seconds_color: Color::from_rgba(245.0 / 255.0, 40.0 / 255.0, 145.0 / 255.0, 0.4),
                text_color: Color::WHITE,
                is_settings_open: false,
                color_r: 125,
                color_g: 125,
                color_b: 125,
                opacity: 1.0,
                temp_color: Color::from_rgba(245.0 / 255.0, 250.0 / 255.0, 145.0 / 255.0, 0.4),
                size: Size {
                    height: 400.0,
                    width: 400.0,
                },
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
                if self.is_settings_open {
                    iced::window::resize(
                        iced::window::Id::MAIN,
                        Size {
                            height: 400.0,
                            width: 800.0,
                        },
                    )
                } else {
                    iced::window::resize(
                        iced::window::Id::MAIN,
                        Size {
                            height: self.size.height,
                            width: self.size.width,
                        },
                    )
                }
            }
            Message::Resize => iced::window::resize(iced::window::Id::MAIN, self.size),
            Message::ColorRed(c) => {
                self.color_r = c;
                // println!("{:?}", c);
                self.temp_color = Color::from_rgba(
                    self.color_r as f32 / 255.0,
                    self.color_g as f32 / 255.0,
                    self.color_b as f32 / 255.0,
                    self.opacity,
                );

                Command::none()
            }
            Message::ColorGreen(g) => {
                self.color_g = g;
                // println!("{:?}", c);
                self.temp_color = Color::from_rgba(
                    self.color_r as f32 / 255.0,
                    self.color_g as f32 / 255.0,
                    self.color_b as f32 / 255.0,
                    self.opacity,
                );

                Command::none()
            }
            Message::ColorBlue(b) => {
                self.color_b = b;
                // println!("{:?}", c);
                self.temp_color = Color::from_rgba(
                    self.color_r as f32 / 255.0,
                    self.color_g as f32 / 255.0,
                    self.color_b as f32 / 255.0,
                    self.opacity,
                );
                Command::none()
            }
            Message::Opacity(o) => {
                self.opacity = o;
                // println!("{:?}", c);
                self.temp_color = Color::from_rgba(
                    self.color_r as f32 / 255.0,
                    self.color_g as f32 / 255.0,
                    self.color_b as f32 / 255.0,
                    self.opacity,
                );
                Command::none()
            }
            Message::Seconds => {
                self.temp_color = Color::from_rgba(
                    self.color_r as f32 / 255.0,
                    self.color_g as f32 / 255.0,
                    self.color_b as f32 / 255.0,
                    self.opacity,
                );

                self.seconds_color = self.temp_color;

                Command::none()
            }
            Message::Minutes => {
                self.temp_color = Color::from_rgba(
                    self.color_r as f32 / 255.0,
                    self.color_g as f32 / 255.0,
                    self.color_b as f32 / 255.0,
                    self.opacity,
                );
                self.minutes_color = self.temp_color;
                Command::none()
            }
            Message::Hours => {
                self.temp_color = Color::from_rgba(
                    self.color_r as f32 / 255.0,
                    self.color_g as f32 / 255.0,
                    self.color_b as f32 / 255.0,
                    self.opacity,
                );
                self.hours_color = self.temp_color;
                Command::none()
            }
            Message::Width(w) => {
                self.size = Size {
                    width: w,
                    height: w,
                };
                // self.size.width = w;
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
            .width(Length::Fill)
            .height(Length::Fill);
        let h_slider = row![
            text("R"),
            slider(0..=255, self.color_r, Message::ColorRed),
            text(self.color_r)
        ]
        .width(250);
        let g_slider = row![
            text("G"),
            slider(0..=255, self.color_g, Message::ColorGreen),
            text(self.color_g)
        ]
        .width(250);
        let b_slider = row![
            text("B"),
            slider(0..=255, self.color_b, Message::ColorBlue),
            text(self.color_b)
        ]
        .width(250);
        let o_slider = row![
            text("O"),
            slider(0.0..=1.0, self.opacity, Message::Opacity)
                .step(0.1)
                .shift_step(0.1),
            text(self.opacity)
        ]
        .width(250);
        let width_slider = row![
            text("width"),
            slider(0.0..=1000.0, self.size.width, Message::Width)
                .step(1.0)
                .shift_step(1.0),
            text(self.size.width)
        ]
        .width(250);
        let main_row = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(cc)
            .push_maybe(
                self.is_settings_open.then_some(
                    container(
                        column![
                            container(text(""))
                                .style(|_style| { self.temp_color.into() })
                                .width(Length::Fill)
                                .height(30),
                            h_slider,
                            g_slider,
                            b_slider,
                            o_slider,
                            width_slider,
                            row![
                                button("Seconds").on_press(Message::Seconds),
                                button("Minutes").on_press(Message::Minutes),
                                button("Hours").on_press(Message::Hours),
                            ]
                            .spacing(10),
                            button("resize").on_press(Message::Resize)
                        ]
                        .spacing(10)
                        .align_items(iced::Alignment::Center)
                        .width(Length::Fill)
                        .height(Length::Fill),
                    )
                    .style(|_style| Color::from_rgba(0.1, 0.3, 0.4, 0.9).into()),
                ),
            );
        let main_container = container(main_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_style| {
                Border::rounded(10);
                Color::TRANSPARENT.into()
            });

        main_container.into()
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
            let minute_path = Path::circle(center, radius - bar_height * 2.0);
            let hour_path = Path::circle(center, radius - (bar_height * 2.0) - bar_height);

            frame.stroke(
                &second_path,
                canvas::Stroke::default()
                    .with_color(self.seconds_color)
                    .with_width(bar_height),
            );
            frame.stroke(
                &minute_path,
                canvas::Stroke::default()
                    .with_color(self.minutes_color)
                    .with_width(bar_height),
            );
            frame.stroke(
                &hour_path,
                canvas::Stroke::default()
                    .with_color(self.hours_color)
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
            let mut minclr = self.minutes_color;
            minclr.a = 1.0;
            let mut hrclr = self.hours_color;
            hrclr.a = 1.0;

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
                    .with_color(minclr)
                    .with_width(bar_height),
            );
            frame.stroke(
                &hours_path_bd,
                canvas::Stroke::default()
                    .with_color(hrclr)
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
