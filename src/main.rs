use iced:: {Application, executor, Command, Element, Text, Settings};


fn main() {
    Theme::run(Settings::default());
}

struct Theme;
impl Application for Theme {
    type Executor = executor::Null;
    type Message = ();
    type Flags = ();

    fn new(_flags: ())-> (Theme, Command<Self::Message>){
        (Theme, Command::none())
    }
    fn title(&self)-> String{
        String::from("Koompi Theme")
    }
    fn update(&mut self, _message: Self::Message)-> Command<Self::Message>{
        Command::none()
    }
    fn view (&mut self)-> Element<Self::Message>{
        
        Text::new("Hello, World!").into()
    }   
}

