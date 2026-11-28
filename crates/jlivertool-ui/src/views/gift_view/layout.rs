use super::{GiftEntry, GiftView};
use crate::theme::Colors;
use chrono::{Local, TimeZone};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::{Input, InputEvent, InputState, MaskPattern};
use gpui_component::menu::{DropdownMenu, PopupMenuItem};
use gpui_component::scroll::Scrollbar;
use gpui_component::switch::Switch;
use gpui_component::{h_flex, v_flex, Selectable, Sizable};
use jlivertool_core::types::guard_level_name;
use std::time::{Duration, Instant};

const DELETE_CONFIRM_TIMEOUT: Duration = Duration::from_secs(3);

/// Only one record in the displayed list can be awaiting a second click.
#[derive(Default)]
struct DeleteConfirmation {
    armed: Option<PendingDelete>,
}

struct PendingDelete {
    id: SharedString,
    expires_at: Instant,
    _timeout: Task<()>,
    _activation: Subscription,
}

impl DeleteConfirmation {
    fn is_armed(&self, id: &str, cx: &App) -> bool {
        self.armed.as_ref().is_some_and(|pending| {
            pending.id == id && cx.background_executor().now() < pending.expires_at
        })
    }

    fn cancel(&mut self, cx: &mut Context<Self>) {
        if self.armed.take().is_some() {
            cx.notify();
        }
    }

    fn activate(&mut self, id: &SharedString, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if self.is_armed(id, cx) {
            self.cancel(cx);
            return true;
        }

        let expires_at = cx.background_executor().now() + DELETE_CONFIRM_TIMEOUT;
        let delay = cx.background_executor().timer(DELETE_CONFIRM_TIMEOUT);
        let timeout = cx.spawn(async move |this, cx| {
            delay.await;
            let _ = this.update(cx, Self::cancel);
        });
        self.armed = Some(PendingDelete {
            id: id.clone(),
            expires_at,
            _timeout: timeout,
            _activation: cx.observe_window_activation(window, |this, _, cx| this.cancel(cx)),
        });
        cx.notify();
        false
    }
}

/// The same gift view can move between a standalone window and the dashboard.
/// Keep filter values on the view, but create the editors in their host window.
pub(super) struct AmountInputs {
    window_id: WindowId,
    min: Entity<InputState>,
    max: Entity<InputState>,
    _subscriptions: [Subscription; 2],
}

impl AmountInputs {
    fn new(min: f64, max: f64, window: &mut Window, cx: &mut Context<GiftView>) -> Self {
        let mut field = |placeholder: &'static str, amount: f64| {
            cx.new(|cx| {
                let mut input = InputState::new(window, cx)
                    .placeholder(placeholder)
                    .mask_pattern(MaskPattern::number(None));
                if amount > 0.0 {
                    input.set_value(amount.to_string(), window, cx);
                }
                input
            })
        };
        let min = field("最低", min);
        let max = field("最高", max);
        let subscriptions = [&min, &max].map(|field| {
            cx.subscribe(field, |view, _, event, cx| {
                if matches!(event, InputEvent::Change) {
                    if let Some(inputs) = &view.amount_inputs {
                        view.min_value = inputs.min.read(cx).value().parse().unwrap_or_default();
                        view.max_value = inputs.max.read(cx).value().parse().unwrap_or_default();
                        view.invalidate_cache();
                        cx.notify();
                    }
                }
            })
        });
        Self {
            window_id: window.window_handle().window_id(),
            min,
            max,
            _subscriptions: subscriptions,
        }
    }
}

impl GiftView {
    fn render_toolbar(&self, count: usize, cx: &Context<Self>) -> impl IntoElement {
        let active = self.only_guards as usize
            + self.show_archived as usize
            + (self.min_value > 0.0 || self.max_value > 0.0) as usize;
        let view = cx.entity();
        h_flex()
            .w_full()
            .h(px(40.0))
            .flex_shrink_0()
            .px_3()
            .gap_2()
            .border_b_1()
            .border_color(Colors::border().opacity(0.5))
            .bg(Colors::bg_secondary_with_opacity(self.opacity))
            .child(
                h_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_2()
                    .child(
                        div()
                            .truncate()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("本地记录"),
                    )
                    .child(
                        div()
                            .flex_shrink_0()
                            .text_size(px(11.0))
                            .text_color(Colors::text_secondary())
                            .child(format!("{count} 条")),
                    ),
            )
            .child(
                Button::new("gift-filters")
                    .debug_selector(|| "gift-filters".into())
                    .small()
                    .ghost()
                    .label(if active == 0 {
                        "筛选".into()
                    } else {
                        format!("筛选（{active}）")
                    })
                    .toggled(self.filters_expanded)
                    .selected(self.filters_expanded)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.filters_expanded = !this.filters_expanded;
                        cx.notify();
                    })),
            )
            .child(
                Button::new("gift-more")
                    .debug_selector(|| "gift-more".into())
                    .small()
                    .ghost()
                    .label("更多")
                    .dropdown_caret(true)
                    .dropdown_menu_with_anchor(Anchor::TopRight, move |menu, window, cx| {
                        let logged_in = view.read(cx).online_account.is_some();
                        menu.item(
                            PopupMenuItem::new(if logged_in {
                                "查询 B 站记录"
                            } else {
                                "查询 B 站记录（需登录）"
                            })
                            .disabled(!logged_in)
                            .on_click(
                                window.listener_for(&view, |this, _, _, cx| {
                                    this.open_online_query(cx)
                                }),
                            ),
                        )
                        .separator()
                        .item(
                            PopupMenuItem::new("清空本地记录…").on_click(
                                window.listener_for(&view, |this, _, window, cx| {
                                    this.confirm_clear(window, cx)
                                }),
                            ),
                        )
                    }),
            )
    }

    fn confirm_clear(&self, window: &mut Window, cx: &mut Context<Self>) {
        let room_id = self.room_id;
        let answer = window.prompt(
            PromptLevel::Warning,
            "清空当前直播间的本地记录？",
            Some("所有礼物和舰长记录（包括已归档记录）都会被删除，无法恢复。"),
            &[PromptButton::cancel("取消"), PromptButton::new("清空")],
            cx,
        );
        cx.spawn(async move |view, cx| {
            if matches!(answer.await, Ok(1)) {
                let _ = view.update(cx, |view, cx| {
                    if view.room_id == room_id {
                        view.clear_all(cx);
                    }
                });
            }
        })
        .detach();
    }

    fn render_filters(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window_id = window.window_handle().window_id();
        let inputs = match &self.amount_inputs {
            Some(inputs) if inputs.window_id == window_id => inputs,
            _ => self.amount_inputs.insert(AmountInputs::new(
                self.min_value,
                self.max_value,
                window,
                cx,
            )),
        };
        v_flex()
            .flex_shrink_0()
            .w_full()
            .px_3()
            .py_2()
            .gap_2()
            .bg(Colors::bg_secondary_with_opacity(self.opacity))
            .border_b_1()
            .border_color(Colors::border().opacity(0.5))
            .child(
                h_flex()
                    .flex_wrap()
                    .gap_4()
                    .child(
                        Switch::new("only-guards-toggle")
                            .small()
                            .label("仅舰长")
                            .checked(self.only_guards)
                            .on_change(cx.listener(|this, checked: &bool, _, cx| {
                                this.only_guards = *checked;
                                this.invalidate_cache();
                                cx.notify();
                            })),
                    )
                    .child(
                        Switch::new("show-archived-toggle")
                            .small()
                            .label("包含已归档")
                            .checked(self.show_archived)
                            .on_change(cx.listener(|this, checked: &bool, _, cx| {
                                this.show_archived = *checked;
                                this.invalidate_cache();
                                cx.notify();
                            })),
                    ),
            )
            .child(
                h_flex()
                    .w_full()
                    .max_w(px(340.0))
                    .gap_2()
                    .text_size(px(12.0))
                    .text_color(Colors::text_secondary())
                    .child(div().flex_shrink_0().child("金额（元）"))
                    .child(
                        div()
                            .debug_selector(|| "gift-min".into())
                            .flex_1()
                            .min_w_0()
                            .child(Input::new(&inputs.min).small()),
                    )
                    .child("至")
                    .child(
                        div()
                            .debug_selector(|| "gift-max".into())
                            .flex_1()
                            .min_w_0()
                            .child(Input::new(&inputs.max).small()),
                    ),
            )
    }

    fn render_empty(&self) -> impl IntoElement {
        let (title, hint) = if self.gift_list.is_empty() {
            ("暂无本地记录", "收到的礼物和舰长记录会显示在这里")
        } else {
            (
                "没有符合条件的记录",
                "可在「筛选」中调整金额、类型或归档条件",
            )
        };
        v_flex()
            .flex_1()
            .min_h_0()
            .items_center()
            .justify_center()
            .gap_1()
            .child(div().font_weight(FontWeight::MEDIUM).child(title))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(Colors::text_secondary())
                    .child(hint),
            )
    }

    fn render_list(
        &self,
        entries: Vec<GiftEntry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let view = cx.entity();
        let opacity = self.opacity;
        // A different window, room, or remounted dashboard panel starts unarmed.
        let confirmation = window.use_keyed_state(
            ("gift-delete", self.room_id.unwrap_or_default()),
            cx,
            |_, _| DeleteConfirmation::default(),
        );
        h_flex()
            .id("gift-container")
            .flex_1()
            .min_h_0()
            .w_full()
            .overflow_hidden()
            .when(confirmation.read(cx).armed.is_some(), |el| {
                el.on_scroll_wheel(
                    window.listener_for(&confirmation, |this, _, _, cx| this.cancel(cx)),
                )
                .capture_key_down(window.listener_for(
                    &confirmation,
                    |this, event: &KeyDownEvent, _, cx| {
                        if !matches!(event.keystroke.key.as_str(), "enter" | "space") {
                            this.cancel(cx);
                        }
                    },
                ))
            })
            .child(
                uniform_list("gift-list", entries.len(), move |range, window, cx| {
                    range
                        .map(|ix| {
                            let entry = &entries[ix];
                            let actions = row_actions(entry, &view, &confirmation, window, cx);
                            render_row(entry, opacity, actions)
                        })
                        .collect()
                })
                .flex_1()
                .min_w_0()
                .h_full()
                .track_scroll(&self.scroll_handle),
            )
            .child(Scrollbar::vertical(&self.scroll_handle))
    }
}

impl Render for GiftView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entries = self.filtered_gift_list();
        v_flex()
            .size_full()
            .bg(Colors::bg_primary_with_opacity(self.opacity))
            .text_size(px(13.0))
            .text_color(Colors::text_primary())
            .child(self.render_toolbar(entries.len(), cx))
            .when(self.filters_expanded, |el| {
                el.child(self.render_filters(window, cx))
            })
            .child(if entries.is_empty() {
                self.render_empty().into_any_element()
            } else {
                self.render_list(entries, window, cx).into_any_element()
            })
    }
}

fn render_row(entry: &GiftEntry, opacity: f32, actions: impl IntoElement) -> impl IntoElement {
    let (action, name, quantity, price) = match entry {
        GiftEntry::Gift(g) => (
            g.action.as_str(),
            g.gift_info.name.as_str(),
            format!("×{}", g.num),
            if g.gift_info.coin_type == "silver" {
                "免费".to_owned()
            } else {
                let unit = if g.num > 1 { "/个" } else { "" };
                format!("¥{:.2}{unit}", g.gift_info.price as f64 / 1000.0)
            },
        ),
        GiftEntry::Guard(g) => (
            "开通了",
            guard_level_name(g.guard_level),
            format!("{}{}", g.num, g.unit),
            format!("¥{:.2}", g.price as f64 / 1000.0),
        ),
    };
    let archived = entry.archived();
    let time = Local
        .timestamp_opt(entry.timestamp(), 0)
        .single()
        .map(|dt| dt.format("%Y/%m/%d %H:%M:%S").to_string())
        .unwrap_or_default();
    let secondary = Colors::text_secondary();
    v_flex()
        .w_full()
        .h(px(56.0))
        .justify_center()
        .gap(px(2.0))
        .px_3()
        .border_b_1()
        .border_color(Colors::border().opacity(0.28 * opacity))
        .when(!archived, |el| match entry {
            GiftEntry::Guard(g) => {
                el.bg(Colors::guard_color(g.guard_level).opacity(0.08 * opacity))
            }
            _ => el,
        })
        .hover(|s| s.bg(Colors::bg_hover_with_opacity(opacity)))
        .child(
            h_flex()
                .w_full()
                .gap_2()
                .child(
                    h_flex()
                        .flex_1()
                        .min_w_0()
                        .gap_1()
                        .child(
                            div()
                                .min_w_0()
                                .truncate()
                                .font_weight(FontWeight::SEMIBOLD)
                                .when(archived, |el| el.text_color(secondary))
                                .child(entry.sender_name().to_owned()),
                        )
                        .child(
                            div()
                                .flex_shrink_0()
                                .max_w(relative(0.18))
                                .truncate()
                                .text_size(px(12.0))
                                .text_color(secondary)
                                .child(action.to_owned()),
                        )
                        .child(
                            div()
                                .flex_shrink_0()
                                .max_w(relative(0.45))
                                .truncate()
                                .text_color(if archived {
                                    secondary
                                } else {
                                    Colors::accent()
                                })
                                .child(name.to_owned()),
                        )
                        .child(
                            div()
                                .flex_shrink_0()
                                .text_size(px(12.0))
                                .text_color(secondary)
                                .child(quantity),
                        ),
                )
                .child(
                    div()
                        .flex_shrink_0()
                        .font_weight(FontWeight::MEDIUM)
                        .when(archived, |el| el.text_color(secondary))
                        .child(price),
                ),
        )
        .child(
            h_flex()
                .w_full()
                .gap_2()
                .child(
                    h_flex()
                        .flex_1()
                        .min_w_0()
                        .gap_2()
                        .text_size(px(11.0))
                        .text_color(secondary)
                        .child(div().truncate().child(time))
                        .when(archived, |el| {
                            el.child(div().flex_shrink_0().child("已归档"))
                        }),
                )
                .child(actions),
        )
}

fn row_actions(
    entry: &GiftEntry,
    view: &Entity<GiftView>,
    confirmation: &Entity<DeleteConfirmation>,
    window: &Window,
    cx: &App,
) -> impl IntoElement {
    let id = SharedString::from(entry.id().to_owned());
    let archive = if entry.archived() { "恢复" } else { "归档" };
    let armed = confirmation.read(cx).is_armed(&id, cx);
    let delete = if armed { "确认删除" } else { "删除" };
    let button = |kind: &str, label: &'static str| {
        Button::new(SharedString::from(format!("gift-{kind}-{id}")))
            .xsmall()
            .h(px(24.0))
            .ghost()
            .label(label)
            .accessibility_label(format!("{label}{}的记录", entry.sender_name()))
    };
    h_flex()
        .flex_shrink_0()
        .gap_1()
        .child(
            button("archive", archive)
                .debug_selector(|| {
                    if entry.archived() {
                        format!("gift-restore-{id}")
                    } else {
                        format!("gift-archive-{id}")
                    }
                })
                .w(px(44.0))
                .text_color(Colors::text_secondary())
                .on_click(window.listener_for(view, {
                    let id = id.clone();
                    let confirmation = confirmation.clone();
                    move |this, _, _, cx| {
                        confirmation.update(cx, DeleteConfirmation::cancel);
                        this.toggle_archived(&id, cx);
                    }
                })),
        )
        .child(
            button("delete", delete)
                .debug_selector(|| {
                    if armed {
                        format!("gift-confirm-delete-{id}")
                    } else {
                        format!("gift-delete-{id}")
                    }
                })
                // Keep the second click at the same position, even in a narrow panel.
                .w(px(64.0))
                .when_else(
                    armed,
                    |button| {
                        button.danger().on_mouse_down_out(
                            window.listener_for(confirmation, |this, _, _, cx| this.cancel(cx)),
                        )
                    },
                    |button| button.text_color(Colors::text_secondary()),
                )
                .on_click(window.listener_for(view, {
                    let confirmation = confirmation.clone();
                    move |this, _, window, cx| {
                        if confirmation.update(cx, |state, cx| state.activate(&id, window, cx)) {
                            this.delete_entry(&id, cx);
                        }
                    }
                })),
        )
}
