use iced::{ Sandbox, Row,  button, Button, Text, Column, Element, Command};
use iced_custom_widget::Grid;
#[derive(Debug,Default)]
pub struct Theme{
    Btn:Vec<(String, button::State)>,
 
}
#[derive(Debug, Clone, Copy)]
pub enum AppMessage { 
    ItemClicked(usize),

}

impl Sandbox for Theme{

    type Message = AppMessage;

    fn new() ->Self{
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Todo List")
    }

    fn update(&mut self, message: Self::Message) {
        // match message {
     
        //     AppMessage::ItemClicked=>{print!("button clicked")}
        // }
    }

    fn view(&mut self) ->  iced::Element<'_, Self::Message> {

        let mut items = vec![
            ("button1".to_string(), button::State::new()),
            ("button2".to_string(), button::State::new()),
            ("button3".to_string(), button::State::new()),
            ("button4".to_string(), button::State::new()),
            ("button5".to_string(), button::State::new()),
            ("button6".to_string(), button::State::new()),
        ];

        let menu: Element<_> = items.iter_mut().enumerate()
            .fold(Grid::new().column_width(175), |grid, (i,(title,state))| {
                grid.push(Button::new(state, Text::new(title.as_str())).on_press(AppMessage::ItemClicked(i)))
            }).into();

        menu
        
    }
    
}
