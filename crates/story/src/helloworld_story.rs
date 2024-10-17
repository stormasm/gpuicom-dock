use gpui::{ParentElement, Render, Styled, View, ViewContext, VisualContext as _, WindowContext};

use ui::{h_flex, label::Label, v_flex};

pub struct HwStory {
    focus_handle: gpui::FocusHandle,
}

impl HwStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(Self::new)
    }

    fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl super::Story for HwStory {
    fn title() -> &'static str {
        "Hw"
    }

    fn new_view(cx: &mut WindowContext) -> View<impl gpui::FocusableView> {
        Self::view(cx)
    }

    fn zoomable() -> bool {
        false
    }
}
impl gpui::FocusableView for HwStory {
    fn focus_handle(&self, _: &gpui::AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl Render for HwStory {
    fn render(&mut self, _: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        v_flex()
            .p_4()
            .gap_5()
            .child(h_flex().justify_center().child(Label::new("Hello World")))
    }
}
