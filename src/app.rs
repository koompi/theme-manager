use button::State;
use iced::{Button, Column, Container, Element, Length, Row, Sandbox, Svg, Text, button, futures::task::Spawn,Space};
use iced_custom_widget::Grid;
use crate::styles;

use super::styles::CustomButton;

#[derive(Debug,Default)]        
pub struct Theme{
    themes: Vec<(&'static str, &'static str, button::State)>,
    selected_theme: usize,
    apply_btn:button::State,
    cancel_btn:button::State,

}

#[derive(Debug, Clone, Copy)]
pub enum AppMessage { 
    ItemClicked(usize),
    ApplyBtn,
    CancelBtn,
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
            selected_theme: 0,
            apply_btn:State::new(),
            cancel_btn: State::new(),

        }
    }

    fn title(&self) -> String {
        String::from("Todo List")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::ItemClicked(idx) => self.selected_theme = idx,
            AppMessage::ApplyBtn =>{
                print!("Apply Clicked")
            },
            AppMessage::CancelBtn =>{
                print!("Cancel Clicked")
            },
        }
    }

    fn view(&mut self) ->  iced::Element<'_, Self::Message> {
        let Theme {
            themes,
            selected_theme,
            apply_btn,
            cancel_btn
        } = self;

        let themes: Element<_> = themes.iter_mut().enumerate()
            .fold(Grid::new().columns(3).spacing(100), |grid, (i, (path, title,state))| {
                let content = Container::new(
                    Column::new().spacing(14).align_items(iced::Align::Center)
                    .push(Svg::from_path(*path).width(Length::Units(75)).height(Length::Units(75)))
                    .push(Text::new(*title))
                ).width(Length::Units(125)).center_x();
                grid.push(
                    Button::new(state, content).padding(15).on_press(AppMessage::ItemClicked(i)).style(if *selected_theme == i {CustomButton::SelectedCard} else {CustomButton::Card})
                )
            }).into();
        let themes_con = Container::new(themes).width(Length::Fill).center_x();
        
        let btn_group = Row::new().width(Length::Fill).spacing(20)
        .push(Space::with_width(Length::Fill))
        .push(
            Button::new (
                &mut self.apply_btn, 
                Text::new("Add")).on_press(AppMessage::ApplyBtn),  
        )
        .push(
            Button::new (
                &mut self.cancel_btn, Text::new("Cancel")).on_press(AppMessage::CancelBtn),
        );
        
        let main_col = Column::new().width(Length::Fill).spacing(15)
            .push(themes_con)
            .push(btn_group);
        Container::new(main_col).width(Length::Fill).height(Length::Fill).padding(30).center_x().center_y().into()
    }
}
