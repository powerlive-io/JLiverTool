//! Gift window view

mod layout;
#[cfg(test)]
mod tests;

use crate::app::UiCommand;
use crate::views::online_gift_view::OnlineGiftView;
use crate::views::window_wrapper::WindowFrameContent;
use gpui::*;
use jlivertool_core::database::Database;
use jlivertool_core::messages::{GiftMessage, GuardMessage};
use layout::AmountInputs;
use std::collections::VecDeque;
use std::sync::Arc;

const MAX_GIFT_COUNT: usize = 500;

#[derive(Debug, Clone)]
pub enum GiftEntry {
    Gift(GiftMessage),
    Guard(GuardMessage),
}

impl GiftEntry {
    fn timestamp(&self) -> i64 {
        match self {
            GiftEntry::Gift(g) => g.timestamp,
            GiftEntry::Guard(g) => g.timestamp,
        }
    }

    fn id(&self) -> &str {
        match self {
            GiftEntry::Gift(g) => &g.id,
            GiftEntry::Guard(g) => &g.id,
        }
    }

    fn sender_name(&self) -> &str {
        match self {
            GiftEntry::Gift(g) => &g.sender.uname,
            GiftEntry::Guard(g) => &g.sender.uname,
        }
    }

    fn archived(&self) -> bool {
        match self {
            GiftEntry::Gift(g) => g.archived,
            GiftEntry::Guard(g) => g.archived,
        }
    }

    fn set_archived(&mut self, archived: bool) {
        match self {
            GiftEntry::Gift(g) => g.archived = archived,
            GiftEntry::Guard(g) => g.archived = archived,
        }
    }

    /// Get the value in CNY (yuan)
    fn value_cny(&self) -> f64 {
        match self {
            GiftEntry::Gift(g) => {
                if g.gift_info.coin_type == "silver" {
                    0.0
                } else {
                    (g.gift_info.price * g.num as u64) as f64 / 1000.0
                }
            }
            GiftEntry::Guard(g) => g.price as f64 / 1000.0,
        }
    }
}

pub struct GiftView {
    online_query: Option<Entity<OnlineGiftView>>,
    online_window: Option<AnyWindowHandle>,
    online_always_on_top: bool,
    online_account: Option<(u64, String)>,
    gift_list: VecDeque<GiftEntry>,
    only_guards: bool,   // Whether to show only guard entries
    show_archived: bool, // Whether to show archived entries
    min_value: f64,      // Minimum value filter in CNY
    max_value: f64,      // Maximum value filter in CNY (0 = no limit)
    filters_expanded: bool,
    amount_inputs: Option<AmountInputs>,
    scroll_handle: UniformListScrollHandle,
    opacity: f32,
    database: Option<Arc<Database>>,
    room_id: Option<u64>,
    // Cached filtered list to avoid O(n) filter on every render
    filtered_cache: Option<Vec<GiftEntry>>,
    // Version counter to track when cache needs invalidation
    list_version: u64,
    cached_list_version: u64,
}

impl GiftView {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            online_query: None,
            online_window: None,
            online_always_on_top: false,
            online_account: None,
            gift_list: VecDeque::with_capacity(MAX_GIFT_COUNT),
            only_guards: false,
            show_archived: false,
            min_value: 0.0,
            max_value: 0.0,
            filters_expanded: false,
            amount_inputs: None,
            scroll_handle: UniformListScrollHandle::new(),
            opacity: 1.0,
            database: None,
            room_id: None,
            filtered_cache: None,
            list_version: 0,
            cached_list_version: 0,
        }
    }

    pub fn set_query_command_sender(
        &mut self,
        sender: std::sync::mpsc::Sender<UiCommand>,
        cx: &mut Context<Self>,
    ) {
        self.online_query = Some(cx.new(|_| OnlineGiftView::new(sender)));
    }

    pub fn set_online_account(&mut self, account: Option<(u64, String)>, cx: &mut Context<Self>) {
        self.online_account = account.clone();
        if let Some(query) = &self.online_query {
            query.update(cx, |view, cx| view.set_account(account, cx));
        }
        cx.notify();
    }

    pub fn set_online_query_always_on_top(&mut self, always_on_top: bool, cx: &mut Context<Self>) {
        self.online_always_on_top = always_on_top;
        if let Some(handle) = self.online_window {
            if cx
                .update_window(handle, |_, window, _| {
                    crate::platform::set_window_always_on_top(window, always_on_top);
                })
                .is_err()
            {
                self.online_window = None;
            }
        }
    }

    pub fn receive_online_gift_types(
        &mut self,
        request_id: u64,
        account_uid: u64,
        result: Result<Vec<jlivertool_core::bilibili::api::GiftStreamCategory>, String>,
        cx: &mut Context<Self>,
    ) {
        if let Some(query) = &self.online_query {
            query.update(cx, |view, cx| {
                view.receive_types(request_id, account_uid, result, cx)
            });
        }
    }

    pub fn receive_online_gifts(
        &mut self,
        request_id: u64,
        account_uid: u64,
        result: Result<Vec<jlivertool_core::bilibili::api::GiftStreamRecord>, String>,
        cx: &mut Context<Self>,
    ) {
        if let Some(query) = &self.online_query {
            query.update(cx, |view, cx| {
                view.receive(request_id, account_uid, result, cx)
            });
        }
    }

    fn open_online_query(&mut self, cx: &mut Context<Self>) {
        if self.online_account.is_none() {
            return;
        }
        let always_on_top = self.online_always_on_top;
        if let Some(handle) = self.online_window {
            if cx
                .update_window(handle, |_, window, _| {
                    crate::platform::set_window_always_on_top(window, always_on_top);
                    window.activate_window();
                })
                .is_ok()
            {
                return;
            }
            self.online_window = None;
        }
        let Some(query) = self.online_query.clone() else {
            return;
        };
        let bounds = Bounds::centered(None, size(px(900.0), px(640.0)), cx);
        match cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("在线记录查询".into()),
                    appears_transparent: true,
                    ..Default::default()
                }),
                window_min_size: Some(size(px(760.0), px(480.0))),
                ..Default::default()
            },
            |window, cx| {
                if always_on_top {
                    crate::platform::set_window_always_on_top(window, true);
                }
                let frame = cx.new(|_| crate::views::WindowFrame::new(query, "在线记录查询"));
                cx.new(|cx| gpui_component::Root::new(frame, window, cx))
            },
        ) {
            Ok(handle) => self.online_window = Some(handle.into()),
            Err(error) => tracing::error!("Failed to open online gift query window: {error}"),
        }
    }

    /// Invalidate the filtered cache (call when list or filters change)
    fn invalidate_cache(&mut self) {
        self.list_version = self.list_version.wrapping_add(1);
    }

    pub fn add_gift(&mut self, gift: GiftMessage, cx: &mut Context<Self>) {
        // Check for duplicate by ID
        if self.gift_list.iter().any(|e| e.id() == gift.id) {
            return;
        }

        let should_auto_scroll = self.is_at_bottom();

        self.gift_list.push_back(GiftEntry::Gift(gift));
        while self.gift_list.len() > MAX_GIFT_COUNT {
            self.gift_list.pop_front();
        }

        self.invalidate_cache();

        if should_auto_scroll {
            let last_index = self.gift_list.len().saturating_sub(1);
            self.scroll_handle
                .scroll_to_item(last_index, ScrollStrategy::Bottom);
        }

        cx.notify();
    }

    pub fn add_guard(&mut self, guard: GuardMessage, cx: &mut Context<Self>) {
        // Check for duplicate by ID
        if self.gift_list.iter().any(|e| e.id() == guard.id) {
            return;
        }

        let should_auto_scroll = self.is_at_bottom();

        self.gift_list.push_back(GiftEntry::Guard(guard));
        while self.gift_list.len() > MAX_GIFT_COUNT {
            self.gift_list.pop_front();
        }

        self.invalidate_cache();

        if should_auto_scroll {
            let last_index = self.gift_list.len().saturating_sub(1);
            self.scroll_handle
                .scroll_to_item(last_index, ScrollStrategy::Bottom);
        }

        cx.notify();
    }

    /// Filter gift list based on current filters (with caching)
    fn filtered_gift_list(&mut self) -> Vec<GiftEntry> {
        // Return cached result if still valid
        if self.cached_list_version == self.list_version {
            if let Some(ref cached) = self.filtered_cache {
                return cached.clone();
            }
        }

        // Rebuild the filtered list
        let filtered: Vec<GiftEntry> = self
            .gift_list
            .iter()
            .filter(|entry| {
                // Filter out archived entries if not showing them
                if !self.show_archived && entry.archived() {
                    return false;
                }
                // Filter to only guards if enabled
                if self.only_guards && !matches!(entry, GiftEntry::Guard(_)) {
                    return false;
                }
                let value = entry.value_cny();
                let min_ok = value >= self.min_value;
                let max_ok = self.max_value <= 0.0 || value <= self.max_value;
                min_ok && max_ok
            })
            .cloned()
            .collect();

        // Update cache
        self.filtered_cache = Some(filtered.clone());
        self.cached_list_version = self.list_version;

        filtered
    }

    pub fn clear_all(&mut self, cx: &mut Context<Self>) {
        // Delete from database
        if let (Some(db), Some(room_id)) = (&self.database, self.room_id) {
            let _ = db.clear_gifts(room_id);
            let _ = db.clear_guards(room_id);
        }
        self.gift_list.clear();
        self.invalidate_cache();
        cx.notify();
    }

    /// Load historical gifts and guards from database
    pub fn load_from_database(&mut self, db: &Arc<Database>, room_id: u64, cx: &mut Context<Self>) {
        self.database = Some(db.clone());
        self.room_id = Some(room_id);
        self.gift_list.clear();

        // Load recent gifts
        if let Ok(gifts) = db.get_recent_gifts(room_id, MAX_GIFT_COUNT / 2) {
            for gift in gifts {
                self.gift_list.push_back(GiftEntry::Gift(gift));
            }
        }

        // Load recent guards
        if let Ok(guards) = db.get_recent_guards(room_id, MAX_GIFT_COUNT / 2) {
            for guard in guards {
                self.gift_list.push_back(GiftEntry::Guard(guard));
            }
        }

        // Sort by archived status first (archived at front), then by timestamp
        let mut sorted: Vec<GiftEntry> = self.gift_list.drain(..).collect();
        sorted.sort_by(|a, b| {
            // Archived items come first (at the top/front of the list)
            match (a.archived(), b.archived()) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.timestamp().cmp(&b.timestamp()),
            }
        });
        self.gift_list = sorted.into_iter().collect();

        // Trim to max count
        while self.gift_list.len() > MAX_GIFT_COUNT {
            self.gift_list.pop_front();
        }

        self.invalidate_cache();

        // Scroll to bottom
        if !self.gift_list.is_empty() {
            let last_index = self.gift_list.len().saturating_sub(1);
            self.scroll_handle
                .scroll_to_item(last_index, ScrollStrategy::Bottom);
        }

        cx.notify();
    }

    pub fn set_opacity(&mut self, opacity: f32, cx: &mut Context<Self>) {
        self.opacity = opacity;
        cx.notify();
    }

    /// Toggle archived status for an entry by ID
    pub fn toggle_archived(&mut self, id: &str, cx: &mut Context<Self>) {
        // Find the entry and toggle its archived status
        for entry in self.gift_list.iter_mut() {
            if entry.id() == id {
                let new_archived = !entry.archived();
                entry.set_archived(new_archived);

                // Update in database
                if let Some(db) = &self.database {
                    match entry {
                        GiftEntry::Gift(_) => {
                            let _ = db.set_gift_archived(id, new_archived);
                        }
                        GiftEntry::Guard(_) => {
                            let _ = db.set_guard_archived(id, new_archived);
                        }
                    }
                }
                break;
            }
        }

        // Re-sort the list (archived items at front)
        let mut sorted: Vec<GiftEntry> = self.gift_list.drain(..).collect();
        sorted.sort_by(|a, b| match (a.archived(), b.archived()) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.timestamp().cmp(&b.timestamp()),
        });
        self.gift_list = sorted.into_iter().collect();

        self.invalidate_cache();
        cx.notify();
    }

    /// Delete an entry by ID
    pub fn delete_entry(&mut self, id: &str, cx: &mut Context<Self>) {
        // Find and remove the entry
        let mut entry_type = None;
        self.gift_list.retain(|entry| {
            if entry.id() == id {
                entry_type = Some(match entry {
                    GiftEntry::Gift(_) => "gift",
                    GiftEntry::Guard(_) => "guard",
                });
                false
            } else {
                true
            }
        });

        // Delete from database
        if let (Some(db), Some(etype)) = (&self.database, entry_type) {
            match etype {
                "gift" => {
                    let _ = db.delete_gift(id);
                }
                "guard" => {
                    let _ = db.delete_guard(id);
                }
                _ => {}
            }
        }

        self.invalidate_cache();
        cx.notify();
    }

    fn is_at_bottom(&self) -> bool {
        if self.gift_list.len() <= 1 {
            return true;
        }

        let scroll_state = self.scroll_handle.0.borrow();
        let base_handle = &scroll_state.base_handle;
        let offset = base_handle.offset();
        let max_offset = base_handle.max_offset();
        let threshold = px(50.0);
        offset.y <= -max_offset.y + threshold
    }
}

impl WindowFrameContent for GiftView {
    fn window_opacity(&self) -> f32 {
        self.opacity
    }
}
