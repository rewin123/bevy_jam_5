use bevy::{color::Srgba, ui::prelude::*};
use bevy_mod_stylebuilder::{
    StyleBuilder, StyleBuilderBackground, StyleBuilderLayout, StyleBuilderZIndex, StyleHandle,
};
use bevy_quill::{Element, View, ViewTemplate};

#[derive(Clone, PartialEq)]
pub(crate) struct EndScreen {
    pub text: String,
    pub style: StyleHandle,
    pub end_type: EndType,
}

impl Default for EndScreen {
    fn default() -> Self {
        Self {
            text: "You Died".to_string(),
            style: StyleHandle::default(),
            end_type: EndType::default(),
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum EndType {
    Win,
    #[default]
    Lose,
}

impl EndScreen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, l: impl Into<String>) -> Self {
        self.text = l.into();
        self
    }

    // Left for future use
    #[allow(dead_code)]
    pub fn style(mut self, s: StyleHandle) -> Self {
        self.style = s;
        self
    }

    pub const fn end_type(mut self, end_type: EndType) -> Self {
        self.end_type = end_type;
        self
    }
}

fn end_screen_style(sb: &mut StyleBuilder) {
    sb.top(0)
        .position(PositionType::Absolute)
        .z_index(100)
        .left(0)
        .bottom(0)
        .right(0)
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .padding(20.0)
        .display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center)
        .background_color(Srgba::new(0.118, 0.118, 0.133, 0.5));
}

impl ViewTemplate for EndScreen {
    type View = impl View;

    fn create(&self, _: &mut bevy_quill::Cx) -> Self::View {
        let title = if self.end_type == EndType::Lose {
            "You Lose"
        } else {
            "You Wind"
        };
        Element::<NodeBundle>::new()
            .style(end_screen_style)
            .children((title, self.text.clone()))
    }
}
