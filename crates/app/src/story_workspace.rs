use anyhow::{Context, Result};
use gpui::*;
use prelude::FluentBuilder as _;
use serde::Deserialize;
use std::{sync::Arc, time::Duration};
use story::{
    AccordionStory, ButtonStory, CalendarStory, DropdownStory, IconStory, ImageStory, InputStory,
    ListStory, ModalStory, PopupStory, ProgressStory, ResizableStory, ScrollableStory,
    StoryContainer, SwitchStory, TableStory, TextStory, TooltipStory,
};
use ui::{
    button::{Button, ButtonStyled as _},
    color_picker::{ColorPicker, ColorPickerEvent},
    dock::{DockArea, DockAreaState, DockEvent, DockItem, PanelView},
    h_flex,
    popup_menu::PopupMenuExt,
    theme::{ActiveTheme, Theme},
    ContextModal, IconName, Root, Sizable, TitleBar,
};

use crate::app_state::AppState;

const MAIN_DOCK_AREA: DockAreaTab = DockAreaTab {
    id: "main-dock",
    version: 5,
};

#[derive(Clone, PartialEq, Eq, Deserialize)]
struct SelectLocale(SharedString);

impl_actions!(locale_switcher, [SelectLocale]);

actions!(workspace, [Open, CloseWindow]);

pub fn init(_app_state: Arc<AppState>, cx: &mut AppContext) {
    cx.on_action(|_action: &Open, _cx: &mut AppContext| {});

    ui::init(cx);
    story::init(cx);
}

pub struct StoryWorkspace {
    theme_color: Option<Hsla>,
    dock_area: View<DockArea>,
    locale_selector: View<LocaleSelector>,
    theme_color_picker: View<ColorPicker>,
    last_layout_state: Option<DockAreaState>,
    _save_layout_task: Option<Task<()>>,
}

struct DockAreaTab {
    id: &'static str,
    version: usize,
}

impl StoryWorkspace {
    pub fn new(_app_state: Arc<AppState>, cx: &mut ViewContext<Self>) -> Self {
        cx.observe_window_appearance(|_, cx| {
            Theme::sync_system_appearance(cx);
        })
        .detach();

        let dock_area =
            cx.new_view(|cx| DockArea::new(MAIN_DOCK_AREA.id, Some(MAIN_DOCK_AREA.version), cx));
        let weak_dock_area = dock_area.downgrade();

        match Self::load_layout(dock_area.clone(), cx) {
            Ok(_) => {
                println!("load layout success");
            }
            Err(err) => {
                eprintln!("load layout error: {:?}", err);
                Self::reset_default_layout(weak_dock_area, cx);
            }
        };

        cx.subscribe(&dock_area, |this, dock_area, ev: &DockEvent, cx| match ev {
            DockEvent::LayoutChanged => this.save_layout(dock_area, cx),
        })
        .detach();

        cx.on_app_quit({
            let dock_area = dock_area.clone();
            move |cx| {
                let state = dock_area.read(cx).dump(cx);
                cx.background_executor().spawn(async move {
                    // Save layout before quitting
                    Self::save_state(&state).unwrap();
                })
            }
        })
        .detach();

        let locale_selector = cx.new_view(LocaleSelector::new);

        let theme_color_picker = cx.new_view(|cx| {
            let mut picker = ColorPicker::new("theme-color-picker", cx)
                .xsmall()
                .anchor(AnchorCorner::TopRight)
                .label("Theme Color");
            picker.set_value(cx.theme().primary, cx);
            picker
        });
        cx.subscribe(
            &theme_color_picker,
            |this, _, ev: &ColorPickerEvent, cx| match ev {
                ColorPickerEvent::Change(color) => {
                    this.set_theme_color(*color, cx);
                }
            },
        )
        .detach();

        Self {
            theme_color: None,
            dock_area,
            locale_selector,
            theme_color_picker,
            last_layout_state: None,
            _save_layout_task: None,
        }
    }

    fn set_theme_color(&mut self, color: Option<Hsla>, cx: &mut ViewContext<Self>) {
        self.theme_color = color;
        if let Some(color) = self.theme_color {
            let theme = cx.global_mut::<Theme>();
            theme.apply_color(color);
            cx.refresh();
        }
    }

    fn change_color_mode(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        let mode = match cx.theme().mode.is_dark() {
            true => ui::theme::ThemeMode::Light,
            false => ui::theme::ThemeMode::Dark,
        };

        Theme::change(mode, cx);
        self.set_theme_color(self.theme_color, cx);
    }

    fn save_layout(&mut self, dock_area: View<DockArea>, cx: &mut ViewContext<Self>) {
        self._save_layout_task = Some(cx.spawn(|this, mut cx| async move {
            Timer::after(Duration::from_secs(10)).await;

            let _ = cx.update(|cx| {
                let dock_area = dock_area.read(cx);
                let state = dock_area.dump(cx);

                let last_layout_state = this.upgrade().unwrap().read(cx).last_layout_state.clone();
                if Some(&state) == last_layout_state.as_ref() {
                    return;
                }

                Self::save_state(&state).unwrap();
                let _ = this.update(cx, |this, _| {
                    this.last_layout_state = Some(state);
                });
            });
        }));
    }

    fn save_state(state: &DockAreaState) -> Result<()> {
        println!("Save layout...");
        let json = serde_json::to_string_pretty(state)?;
        std::fs::write("layout.json", json)?;
        Ok(())
    }

    fn load_layout(dock_area: View<DockArea>, cx: &mut WindowContext) -> Result<()> {
        let fname = "layout.json";
        let json = std::fs::read_to_string(fname)?;
        let state = serde_json::from_str::<DockAreaState>(&json)?;

        // Check if the saved layout version is different from the current version
        // Notify the user and ask if they want to reset the layout to default.
        if state.version != Some(MAIN_DOCK_AREA.version) {
            let answer = cx.prompt(PromptLevel::Info, "The default main layout has been updated.\nDo you want to reset the layout to default?", None,
                &["Yes", "No"]);

            let weak_dock_area = dock_area.downgrade();
            cx.spawn(|mut cx| async move {
                if answer.await == Ok(0) {
                    _ = cx.update(|cx| {
                        Self::reset_default_layout(weak_dock_area, cx);
                    });
                }
            })
            .detach();
        }

        dock_area.update(cx, |dock_area, cx| {
            dock_area.load(state, cx).context("load layout")?;

            Ok::<(), anyhow::Error>(())
        })
    }

    fn reset_default_layout(dock_area: WeakView<DockArea>, cx: &mut WindowContext) {
        let dock_item = Self::init_default_layout(&dock_area, cx);
        let left_panels: Vec<Arc<dyn PanelView>> =
            vec![Arc::new(StoryContainer::panel::<ListStory>(cx))];

        let bottom_panels: Vec<Arc<dyn PanelView>> = vec![
            Arc::new(StoryContainer::panel::<TooltipStory>(cx)),
            Arc::new(StoryContainer::panel::<IconStory>(cx)),
        ];

        let right_panels: Vec<Arc<dyn PanelView>> =
            vec![Arc::new(StoryContainer::panel::<ImageStory>(cx))];

        _ = dock_area.update(cx, |view, cx| {
            view.set_version(MAIN_DOCK_AREA.version, cx);
            view.set_root(dock_item, cx);
            view.set_left_dock(left_panels, Some(px(350.)), cx);
            view.set_bottom_dock(bottom_panels, Some(px(200.)), cx);
            view.set_right_dock(right_panels, Some(px(320.)), cx);

            Self::save_state(&view.dump(cx)).unwrap();
        });
    }

    fn init_default_layout(dock_area: &WeakView<DockArea>, cx: &mut WindowContext) -> DockItem {
        DockItem::split_with_sizes(
            Axis::Vertical,
            vec![DockItem::tabs(
                vec![
                    Arc::new(StoryContainer::panel::<ButtonStory>(cx)),
                    Arc::new(StoryContainer::panel::<InputStory>(cx)),
                    Arc::new(StoryContainer::panel::<DropdownStory>(cx)),
                    Arc::new(StoryContainer::panel::<TextStory>(cx)),
                    Arc::new(StoryContainer::panel::<ModalStory>(cx)),
                    Arc::new(StoryContainer::panel::<PopupStory>(cx)),
                    Arc::new(StoryContainer::panel::<SwitchStory>(cx)),
                    Arc::new(StoryContainer::panel::<ProgressStory>(cx)),
                    Arc::new(StoryContainer::panel::<TableStory>(cx)),
                    Arc::new(StoryContainer::panel::<ImageStory>(cx)),
                    Arc::new(StoryContainer::panel::<IconStory>(cx)),
                    Arc::new(StoryContainer::panel::<TooltipStory>(cx)),
                    Arc::new(StoryContainer::panel::<ProgressStory>(cx)),
                    Arc::new(StoryContainer::panel::<CalendarStory>(cx)),
                    Arc::new(StoryContainer::panel::<ResizableStory>(cx)),
                    Arc::new(StoryContainer::panel::<ScrollableStory>(cx)),
                    Arc::new(StoryContainer::panel::<AccordionStory>(cx)),
                    // Arc::new(StoryContainer::panel::<WebViewStory>(cx)),
                ],
                None,
                &dock_area,
                cx,
            )],
            vec![None],
            &dock_area,
            cx,
        )
    }

    pub fn new_local(
        app_state: Arc<AppState>,
        cx: &mut AppContext,
    ) -> Task<anyhow::Result<WindowHandle<Root>>> {
        let window_bounds = Bounds::centered(None, size(px(1600.0), px(1200.0)), cx);

        cx.spawn(|mut cx| async move {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(9.0), px(9.0))),
                }),
                window_min_size: Some(gpui::Size {
                    width: px(640.),
                    height: px(480.),
                }),
                kind: WindowKind::Normal,
                ..Default::default()
            };

            let window = cx.open_window(options, |cx| {
                let story_view = cx.new_view(|cx| Self::new(app_state.clone(), cx));
                cx.new_view(|cx| Root::new(story_view.into(), cx))
            })?;

            window
                .update(&mut cx, |_, cx| {
                    cx.activate_window();
                    cx.set_window_title("GPUI App");
                    cx.on_release(|_, _, cx| {
                        // exit app
                        cx.quit();
                    })
                    .detach();
                })
                .expect("failed to update window");

            Ok(window)
        })
    }
}

pub fn open_new(
    app_state: Arc<AppState>,
    cx: &mut AppContext,
    init: impl FnOnce(&mut Root, &mut ViewContext<Root>) + 'static + Send,
) -> Task<()> {
    let task: Task<std::result::Result<WindowHandle<Root>, anyhow::Error>> =
        StoryWorkspace::new_local(app_state, cx);
    cx.spawn(|mut cx| async move {
        if let Some(root) = task.await.ok() {
            root.update(&mut cx, |workspace, cx| init(workspace, cx))
                .expect("failed to init workspace");
        }
    })
}

impl Render for StoryWorkspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let drawer_layer = Root::render_drawer_layer(cx);
        let modal_layer = Root::render_modal_layer(cx);
        let notification_layer = Root::render_notification_layer(cx);
        let notifications_count = cx.notifications().len();

        div()
            .font_family(".SystemUIFont")
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                TitleBar::new()
                    // left side
                    .child(div().flex().items_center().child("GPUI App"))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_end()
                            .px_2()
                            .gap_2()
                            .child(self.theme_color_picker.clone())
                            .child(
                                Button::new("theme-mode")
                                    .map(|this| {
                                        if cx.theme().mode.is_dark() {
                                            this.icon(IconName::Sun)
                                        } else {
                                            this.icon(IconName::Moon)
                                        }
                                    })
                                    .small()
                                    .ghost()
                                    .on_click(cx.listener(Self::change_color_mode)),
                            )
                            .child(self.locale_selector.clone())
                            .child(
                                Button::new("github")
                                    .icon(IconName::GitHub)
                                    .small()
                                    .ghost()
                                    .on_click(|_, cx| {
                                        cx.open_url("https://github.com/huacnlee/gpui-component")
                                    }),
                            )
                            .child(
                                div()
                                    .relative()
                                    .child(
                                        Button::new("bell")
                                            .small()
                                            .ghost()
                                            .compact()
                                            .icon(IconName::Bell),
                                    )
                                    .when(notifications_count > 0, |this| {
                                        this.child(
                                            h_flex()
                                                .absolute()
                                                .rounded_full()
                                                .top(px(-2.))
                                                .right(px(-2.))
                                                .p(px(1.))
                                                .min_w(px(12.))
                                                .bg(ui::red_500())
                                                .text_color(ui::white())
                                                .justify_center()
                                                .text_size(px(10.))
                                                .line_height(relative(1.))
                                                .child(format!("{}", notifications_count.min(99))),
                                        )
                                    }),
                            ),
                    ),
            )
            .child(self.dock_area.clone())
            .children(drawer_layer)
            .children(modal_layer)
            .child(div().absolute().top_8().children(notification_layer))
    }
}

struct LocaleSelector {
    focus_handle: FocusHandle,
}

impl LocaleSelector {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    fn on_select_locale(&mut self, locale: &SelectLocale, cx: &mut ViewContext<Self>) {
        ui::set_locale(&locale.0);
        cx.refresh();
    }
}

impl Render for LocaleSelector {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let locale = ui::locale().to_string();

        div()
            .id("locale-selector")
            .track_focus(&focus_handle)
            .on_action(cx.listener(Self::on_select_locale))
            .child(
                Button::new("btn")
                    .small()
                    .ghost()
                    .icon(IconName::Globe)
                    .popup_menu(move |this, _| {
                        this.menu_with_check(
                            "English",
                            locale == "en",
                            Box::new(SelectLocale("en".into())),
                        )
                        .menu_with_check(
                            "简体中文",
                            locale == "zh-CN",
                            Box::new(SelectLocale("zh-CN".into())),
                        )
                    })
                    .anchor(AnchorCorner::TopRight),
            )
    }
}
