use iced::{ Sandbox, Row,  button, Button, Text, Column, Element, Length, Container, Svg};
use iced_custom_widget::Grid;
use super::styles::CustomButton;

#[derive(Debug,Default)]
pub struct Theme{
    themes: Vec<(&'static str, &'static str, button::State)>,
    selected_theme: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum AppMessage { 
    ItemClicked(usize),
}

impl Sandbox for Theme{
    type Message = AppMessage;

    fn new() ->Self{
        let items = vec![
            ("assets/images/koompi-logo.svg", "Koompi Light", button::State::new()),
            ("assets/images/koompi-logo.svg", "Koompi Dark", button::State::new()),
            ("assets/images/koompi-logo.svg", "Mac Light", button::State::new()),
            ("assets/images/koompi-logo.svg", "Mac Dark", button::State::new()),
            ("assets/images/koompi-logo.svg", "Window Light", button::State::new()),
            ("assets/images/koompi-logo.svg", "Window Dark", button::State::new()),
        ];

        Self {
            themes: items,
            selected_theme: 0
        }
    }

    fn title(&self) -> String {
        String::from("Todo List")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::ItemClicked(idx) => self.selected_theme = idx,
        }
    }

    fn view(&mut self) ->  iced::Element<'_, Self::Message> {
        let Theme {
            themes,
            selected_theme
        } = self;

        let menu: Element<_> = themes.iter_mut().enumerate()
            .fold(Grid::new().columns(3).spacing(50), |grid, (i, (path, title,state))| {
                let content = Container::new(
                    Column::new().spacing(14).align_items(iced::Align::Center)
                    .push(Svg::from_path(*path).width(Length::Units(75)).height(Length::Units(75)))
                    .push(Text::new(*title))
                ).width(Length::Units(125)).center_x();
                grid.push(
                    Button::new(state, content).padding(15).on_press(AppMessage::ItemClicked(i)).style(if *selected_theme == i {CustomButton::SelectedCard} else {CustomButton::Card})
                )
            }).into();

        Container::new(menu).width(Length::Fill).height(Length::Fill).padding(20).center_x().center_y().into()
    }
}
