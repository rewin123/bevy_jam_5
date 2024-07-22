use bevy_mod_stylebuilder::StyleHandle;
use bevy_quill::{Callback, View, ViewTemplate};
use bevy_quill_obsidian::{controls::Button, size::Size, RoundedCorners};

#[derive(Clone, PartialEq)]
pub(crate) struct ActionButton {
    pub label: String,
    pub style: StyleHandle,
    pub on_click: Option<Callback<()>>,
}

impl Default for ActionButton {
    fn default() -> Self {
        Self {
            // Copied from Obsidian slider
            label: "Action".to_string(),
            style: StyleHandle::default(),
            on_click: None,
        }
    }
}

impl ActionButton {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = l.into();
        self
    }

    // Left for future use
    #[allow(dead_code)]
    pub fn style(mut self, s: StyleHandle) -> Self {
        self.style = s;
        self
    }

    pub const fn on_click(mut self, cb: Callback<()>) -> Self {
        self.on_click = Some(cb);
        self
    }
}

impl ViewTemplate for ActionButton {
    type View = impl View;

    fn create(&self, _: &mut bevy_quill::Cx) -> Self::View {
        let mut btn = Button::new()
            .style(self.style.clone())
            .corners(RoundedCorners::All)
            .size(Size::Lg)
            .children(self.label.clone());

        if let Some(cb) = self.on_click {
            btn = btn.on_click(cb);
        }

        btn
    }
}
