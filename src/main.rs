mod app;
mod styles;

use app::*;
use iced::{Settings,Sandbox};

fn main() {
    Theme::run(Settings::default());  
}