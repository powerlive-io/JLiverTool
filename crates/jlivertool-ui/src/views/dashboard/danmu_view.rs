use crate::theme::Colors;
use crate::views::main_view::{
    append_message_rows, DanmuListItemView, DanmuRowOptions, DisplayMessage, RenderRow,
    SelectedUserState,
};
use gpui::*;
use gpui_component::h_flex;
use gpui_component::scroll::Scrollbar;
use std::collections::VecDeque;
use std::rc::Rc;

pub(crate) struct DashboardDanmuView {
    messages: VecDeque<DisplayMessage>,
    /// Number of rendered rows produced by each entry in `messages`.
    /// This keeps front eviction incremental even when one message wraps.
    row_counts: VecDeque<usize>,
    rows: Rc<Vec<RenderRow>>,
    scroll_handle: UniformListScrollHandle,
    pending_scroll_to_bottom: bool,
    row_options: DanmuRowOptions,
    opacity: f32,
    selected_user: SelectedUserState,
    content_width: f32,
}

impl DashboardDanmuView {
    pub(crate) fn new(selected_user: SelectedUserState) -> Self {
        Self {
            messages: VecDeque::new(),
            row_counts: VecDeque::new(),
            rows: Rc::new(Vec::new()),
            scroll_handle: UniformListScrollHandle::new(),
            pending_scroll_to_bottom: true,
            row_options: DanmuRowOptions::default(),
            opacity: 1.0,
            selected_user,
            content_width: 0.0,
        }
    }

    pub(crate) fn replace_messages(
        &mut self,
        messages: &VecDeque<DisplayMessage>,
        cx: &mut Context<Self>,
    ) {
        let should_auto_scroll = self.is_at_bottom();
        self.messages = messages.clone();
        self.rebuild_rows(should_auto_scroll);
        cx.notify();
    }

    pub(crate) fn push_message(
        &mut self,
        message: DisplayMessage,
        removed_from_front: usize,
        cx: &mut Context<Self>,
    ) {
        let should_auto_scroll = self.is_at_bottom();
        let mut rows = Rc::try_unwrap(std::mem::replace(&mut self.rows, Rc::new(Vec::new())))
            .unwrap_or_else(|rows| (*rows).clone());

        let mut rows_to_remove = 0;
        for _ in 0..removed_from_front.min(self.messages.len()) {
            self.messages.pop_front();
            rows_to_remove += self.row_counts.pop_front().unwrap_or(0);
        }
        rows.drain(..rows_to_remove.min(rows.len()));

        let previous_row_count = rows.len();
        append_message_rows(
            &mut rows,
            &message,
            self.content_width - 14.0,
            self.row_options,
        );
        self.row_counts.push_back(rows.len() - previous_row_count);
        self.messages.push_back(message);
        self.rows = Rc::new(rows);
        self.pending_scroll_to_bottom = should_auto_scroll;
        cx.notify();
    }

    pub(crate) fn set_style(
        &mut self,
        row_options: DanmuRowOptions,
        opacity: f32,
        cx: &mut Context<Self>,
    ) {
        let layout_changed = self.row_options != row_options;
        let opacity_changed = (self.opacity - opacity).abs() > f32::EPSILON;
        if !layout_changed && !opacity_changed {
            return;
        }

        let should_auto_scroll = layout_changed && self.is_at_bottom();
        self.row_options = row_options;
        self.opacity = opacity;
        if layout_changed {
            self.rebuild_rows(should_auto_scroll);
        }
        cx.notify();
    }

    fn set_content_width(&mut self, width: f32, cx: &mut Context<Self>) {
        if width <= 0.0 || (self.content_width - width).abs() <= 1.0 {
            return;
        }
        let should_auto_scroll = self.is_at_bottom();
        self.content_width = width;
        self.rebuild_rows(should_auto_scroll);
        cx.notify();
    }

    pub(crate) fn is_at_bottom(&self) -> bool {
        if self.pending_scroll_to_bottom || self.rows.len() <= 1 {
            return true;
        }

        let scroll_state = self.scroll_handle.0.borrow();
        let base_handle = &scroll_state.base_handle;
        let offset = base_handle.offset();
        let max_offset = base_handle.max_offset();
        offset.y <= -max_offset.y + px(50.0)
    }

    fn rebuild_rows(&mut self, should_auto_scroll: bool) {
        let mut rows = Vec::new();
        self.row_counts.clear();
        for message in &self.messages {
            let previous_row_count = rows.len();
            append_message_rows(
                &mut rows,
                message,
                self.content_width - 14.0,
                self.row_options,
            );
            self.row_counts.push_back(rows.len() - previous_row_count);
        }
        self.rows = Rc::new(rows);
        self.pending_scroll_to_bottom = should_auto_scroll;
    }
}

impl Render for DashboardDanmuView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.pending_scroll_to_bottom && !self.rows.is_empty() {
            self.pending_scroll_to_bottom = false;
            self.scroll_handle
                .scroll_to_item(self.rows.len().saturating_sub(1), ScrollStrategy::Bottom);
        }

        let rows = Rc::clone(&self.rows);
        let item_count = rows.len();
        let font_size = self.row_options.font_size;
        let lite_mode = self.row_options.lite_mode;
        let medal_display = self.row_options.medal_display;
        let opacity = self.opacity;
        let selected_user = self.selected_user.clone();
        let scroll_handle = self.scroll_handle.clone();
        let view = cx.entity().clone();

        h_flex()
            .relative()
            .size_full()
            .overflow_hidden()
            .bg(Colors::bg_primary_with_opacity(opacity))
            .child(
                canvas(
                    move |bounds, _, cx| {
                        let width = f32::from(bounds.size.width);
                        view.update(cx, |view, cx| view.set_content_width(width, cx));
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            .child(
                uniform_list("dashboard-danmu-list", item_count, move |range, _, _| {
                    range
                        .map(|ix| {
                            DanmuListItemView::new(
                                rows[ix].clone(),
                                ix,
                                font_size,
                                lite_mode,
                                medal_display,
                                opacity,
                                selected_user.clone(),
                            )
                            .render_element()
                        })
                        .collect()
                })
                .flex_1()
                .h_full()
                .track_scroll(&scroll_handle),
            )
            .child(Scrollbar::vertical(&scroll_handle))
    }
}

#[cfg(test)]
mod tests {
    use super::{DanmuRowOptions, DashboardDanmuView, DisplayMessage, RenderRow};
    use gpui::{AppContext, TestAppContext};
    use jlivertool_core::messages::{DanmuMessage, GiftMessage};
    use serde_json::json;

    #[gpui::test]
    fn hidden_gifts_occupy_no_rows_and_restore_after_front_eviction(cx: &mut TestAppContext) {
        let panel = cx.new(|_| DashboardDanmuView::new(Default::default()));
        let body = json!({"data": {"uid": 1, "uname": "观众", "giftId": 1, "giftName": "一个会换行的礼物"}});
        let gift = DisplayMessage::Gift(GiftMessage::from_raw(&body, 1).unwrap());
        let danmu = DanmuMessage::from_raw(&json!({"info": [[], "你好"]}), None).unwrap();

        panel.update(cx, |panel, cx| {
            let shown = panel.row_options;
            let hidden = DanmuRowOptions {
                gift_display: false,
                ..shown
            };
            panel.set_style(hidden, 1.0, cx);
            panel.push_message(gift.clone(), 0, cx);
            assert!(
                panel.rows.is_empty(),
                "hidden even before the panel is measured"
            );
            panel.push_message(DisplayMessage::Danmu(danmu), 0, cx);
            panel.set_content_width(90.0, cx);
            panel.push_message(gift, 1, cx);
            assert!(matches!(
                panel.rows.as_slice(),
                [RenderRow::Full(DisplayMessage::Danmu(_))]
            ));

            panel.set_style(shown, 1.0, cx);
            let (first, rest) = panel.rows.split_first().unwrap();
            assert!(matches!(first, RenderRow::Full(DisplayMessage::Danmu(_))));
            assert!(rest.len() > 1);
            assert!(rest
                .iter()
                .all(|row| matches!(row, RenderRow::GiftLine { .. })));
        });
    }
}
