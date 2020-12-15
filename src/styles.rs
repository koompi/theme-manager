use iced::{button, Color, Vector};

pub enum CustomButton {
   Card,
   SelectedCard,
}

impl button::StyleSheet for CustomButton {
   fn active(&self) -> button::Style {
      use CustomButton::*;
      button::Style {
         background: match self {
            Card => Color::WHITE.into(),
            SelectedCard => Color {a: 0.3, ..Color::BLACK}.into()
         },
         text_color: Color::BLACK,
         border_radius: 5.0,
         border_color: Color::BLACK,
         border_width: 0.0,
         shadow_offset: Vector::new(0.0, 0.0)
      }
   }

   fn hovered(&self) -> button::Style {
      let active = self.active();

      button::Style {
         background: match self {
            CustomButton::Card =>  Color {a: 0.3, ..Color::BLACK}.into(),
            CustomButton::SelectedCard => active.background,
         },
         ..active
      }
   }
} 