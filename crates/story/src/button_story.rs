use gpui::{
    px, ClickEvent, FocusableView, IntoElement, ParentElement as _, Render, Styled as _, View,
    ViewContext, VisualContext as _, WindowContext,
};

use ui::{
    button::{Button, ButtonCustomStyle, ButtonStyled as _},
    button_group::ButtonGroup,
    checkbox::Checkbox,
    h_flex,
    prelude::FluentBuilder,
    theme::{ActiveTheme, Theme},
    v_flex, Disableable as _, Icon, IconName, Selectable as _, Sizable as _,
};

use crate::section;

pub struct ButtonStory {
    focus_handle: gpui::FocusHandle,
    disabled: bool,
    loading: bool,
    selected: bool,
    compact: bool,
    toggle_multiple: bool,
}

impl ButtonStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| Self {
            focus_handle: cx.focus_handle(),
            disabled: false,
            loading: false,
            selected: false,
            compact: false,
            toggle_multiple: false,
        })
    }

    fn on_click(ev: &ClickEvent, _: &mut WindowContext) {
        println!("Button clicked! {:?}", ev);
    }
}

impl super::Story for ButtonStory {
    fn title() -> &'static str {
        "Button"
    }

    fn description() -> &'static str {
        "Displays a button or a component that looks like a button."
    }

    fn closeable() -> bool {
        false
    }

    fn new_view(cx: &mut WindowContext) -> View<impl gpui::FocusableView> {
        Self::view(cx)
    }
}

impl FocusableView for ButtonStory {
    fn focus_handle(&self, _: &gpui::AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ButtonStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let disabled = self.disabled;
        let loading = self.loading;
        let selected = self.selected;
        let compact = self.compact;
        let toggle_multiple = self.toggle_multiple;

        v_flex()
            .gap_6()
            .child(
                h_flex()
                    .gap_3()
                    .child("State")
                    .child(
                        Checkbox::new("disabled-button")
                            .label("Disabled")
                            .checked(self.disabled)
                            .on_click(cx.listener(|view, _, cx| {
                                view.disabled = !view.disabled;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("loading-button")
                            .label("Loading")
                            .checked(self.loading)
                            .on_click(cx.listener(|view, _, cx| {
                                view.loading = !view.loading;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("selected-button")
                            .label("Selected")
                            .checked(self.selected)
                            .on_click(cx.listener(|view, _, cx| {
                                view.selected = !view.selected;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("compact-button")
                            .label("Compact")
                            .checked(self.compact)
                            .on_click(cx.listener(|view, _, cx| {
                                view.compact = !view.compact;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("shadow-button")
                            .label("Shadow")
                            .checked(cx.theme().shadow)
                            .on_click(cx.listener(|_, _, cx| {
                                let mut theme = cx.theme().clone();
                                theme.shadow = !theme.shadow;
                                cx.set_global::<Theme>(theme);
                                cx.refresh();
                            })),
                    ),
            )
            .child(
                h_flex()
                    .gap_6()
                    .child(
                        section("Normal Button", cx)
                            .child(
                                Button::new("button-1")
                                    .primary()
                                    .label("Primary Button")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-2")
                                    .label("Secondary Button")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-4")
                                    .danger()
                                    .label("Danger Button")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-5")
                                    .outline()
                                    .label("Outline Button")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-5-ghost")
                                    .ghost()
                                    .label("Ghost Button")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-5-link")
                                    .link()
                                    .label("Link Button")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-5-text")
                                    .text()
                                    .label("Text Button")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-6-custom")
                                    .custom(
                                        ButtonCustomStyle::new(cx)
                                            .color(if cx.theme().mode.is_dark() {
                                                ui::green_900()
                                            } else {
                                                ui::green_500()
                                            })
                                            .foreground(if cx.theme().mode.is_dark() {
                                                ui::white()
                                            } else {
                                                ui::black()
                                            })
                                            .border(if cx.theme().mode.is_dark() {
                                                ui::green_800()
                                            } else {
                                                ui::green_600()
                                            })
                                            .hover(if cx.theme().mode.is_dark() {
                                                ui::green_800()
                                            } else {
                                                ui::green_500()
                                            })
                                            .active(if cx.theme().mode.is_dark() {
                                                ui::green_950()
                                            } else {
                                                ui::green_600()
                                            }),
                                    )
                                    .label("Custom Button")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            ),
                    )
                    .child(
                        section("Button with Icon", cx)
                            .child(
                                Button::new("button-icon-1")
                                    .primary()
                                    .label("Confirm")
                                    .icon(IconName::Check)
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-icon-2")
                                    .label("Abort")
                                    .icon(IconName::Close)
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-icon-3")
                                    .label("Maximize")
                                    .icon(Icon::new(IconName::Maximize))
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-icon-4")
                                    .primary()
                                    .child(
                                        h_flex()
                                            .items_center()
                                            .gap_2()
                                            .child("Custom Child")
                                            .child(IconName::ChevronDown)
                                            .child(IconName::Eye),
                                    )
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-icon-5-ghost")
                                    .ghost()
                                    .icon(IconName::Check)
                                    .label("Confirm")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-icon-6-link")
                                    .link()
                                    .icon(IconName::Check)
                                    .label("Link")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-icon-6-text")
                                    .text()
                                    .icon(IconName::Check)
                                    .label("Text Button")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .gap_6()
                    .child(
                        section("Small Size", cx)
                            .child(
                                Button::new("button-6")
                                    .label("Primary Button")
                                    .icon(IconName::Check)
                                    .primary()
                                    .small()
                                    .loading(true)
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-7")
                                    .label("Secondary Button")
                                    .small()
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-8")
                                    .label("Danger Button")
                                    .danger()
                                    .small()
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-8-outline")
                                    .label("Outline Button")
                                    .outline()
                                    .small()
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-8-ghost")
                                    .label("Ghost Button")
                                    .ghost()
                                    .small()
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-8-link")
                                    .label("Link Button")
                                    .link()
                                    .small()
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            ),
                    )
                    .child(
                        section("XSmall Size", cx)
                            .child(
                                Button::new("button-xs-1")
                                    .label("Primary Button")
                                    .primary()
                                    .icon(IconName::Check)
                                    .xsmall()
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-xs-2")
                                    .label("Secondary Button")
                                    .xsmall()
                                    .loading(true)
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-xs-3")
                                    .label("Danger Button")
                                    .danger()
                                    .xsmall()
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-xs-3-ghost")
                                    .label("Ghost Button")
                                    .ghost()
                                    .xsmall()
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-xs-3-outline")
                                    .label("Outline Button")
                                    .outline()
                                    .xsmall()
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-xs-3-link")
                                    .label("Link Button")
                                    .link()
                                    .xsmall()
                                    .disabled(disabled)
                                    .selected(selected)
                                    .loading(loading)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            ),
                    ),
            )
            .child(
                section("Button Group", cx)
                    .child(
                        ButtonGroup::new("button-group")
                            .small()
                            .disabled(disabled)
                            .child(
                                Button::new("button-one")
                                    .label("One")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-two")
                                    .label("Two")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            )
                            .child(
                                Button::new("button-three")
                                    .label("Three")
                                    .disabled(disabled)
                                    .selected(selected)
                                    .when(compact, |this| this.compact())
                                    .on_click(Self::on_click),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Checkbox::new("multiple-button")
                                    .label("Multiple")
                                    .checked(toggle_multiple)
                                    .on_click(cx.listener(|view, _, cx| {
                                        view.toggle_multiple = !view.toggle_multiple;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                ButtonGroup::new("toggle-button-group")
                                    .primary()
                                    .compact()
                                    .multiple(toggle_multiple)
                                    .child(
                                        Button::new("disabled-toggle-button")
                                            .label("Disabled")
                                            .selected(disabled),
                                    )
                                    .child(
                                        Button::new("loading-toggle-button")
                                            .label("Loading")
                                            .selected(loading),
                                    )
                                    .child(
                                        Button::new("selected-toggle-button")
                                            .label("Selected")
                                            .selected(selected),
                                    )
                                    .child(
                                        Button::new("compact-toggle-button")
                                            .label("Compact")
                                            .selected(compact),
                                    )
                                    .on_click(cx.listener(|view, selected: &Vec<usize>, cx| {
                                        view.disabled = selected.contains(&0);
                                        view.loading = selected.contains(&1);
                                        view.selected = selected.contains(&2);
                                        view.compact = selected.contains(&3);
                                        cx.notify();
                                    })),
                            ),
                    ),
            )
            .child(
                section("Icon Button", cx)
                    .child(
                        Button::new("icon-button-primary")
                            .icon(IconName::Search)
                            .loading_icon(IconName::LoaderCircle)
                            .primary()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Button::new("icon-button-secondary")
                            .icon(IconName::Info)
                            .loading(true)
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Button::new("icon-button-danger")
                            .icon(IconName::Close)
                            .danger()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Button::new("icon-button-small-primary")
                            .icon(IconName::Search)
                            .small()
                            .primary()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Button::new("icon-button-outline")
                            .icon(IconName::Search)
                            .outline()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Button::new("icon-button-ghost")
                            .icon(IconName::ArrowLeft)
                            .loading_icon(IconName::LoaderCircle)
                            .ghost()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    ),
            )
            .child(
                section("Icon Button", cx)
                    .child(
                        Button::new("icon-button-4")
                            .icon(IconName::Info)
                            .small()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Button::new("icon-button-5")
                            .icon(IconName::Close)
                            .small()
                            .danger()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Button::new("icon-button-6")
                            .icon(IconName::Search)
                            .small()
                            .primary()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Button::new("icon-button-7")
                            .icon(IconName::Info)
                            .xsmall()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Button::new("icon-button-8")
                            .icon(IconName::Close)
                            .xsmall()
                            .danger()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Button::new("icon-button-9")
                            .icon(IconName::Heart)
                            .size(px(24.))
                            .ghost()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    ),
            )
    }
}
