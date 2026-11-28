use super::{GiftEntry, GiftView};
use gpui::{
    point, px, size, AppContext, Entity, Modifiers, MouseButton, MouseDownEvent, MouseUpEvent,
    ScrollDelta, ScrollWheelEvent, TestAppContext, VisualTestContext,
};
use gpui_component::Root;
use jlivertool_core::database::Database;
use jlivertool_core::messages::{GiftMessage, GuardMessage};
use serde_json::json;
use std::{sync::Arc, time::Duration};

fn sample_records(cx: &mut TestAppContext) -> Entity<GiftView> {
    cx.update(gpui_component::init);
    let view = cx.new(GiftView::new);
    view.update(cx, |view, cx| {
        let mut gift = GiftMessage::from_raw(
            &json!({"data": {"uid": 1, "uname": "观众", "giftName": "花", "giftId": 1, "price": 500, "num": 2}}),
            1,
        )
        .unwrap();
        gift.id = "flower".into();
        view.add_gift(gift, cx);
        view.add_guard(
            GuardMessage {
                id: "captain".into(),
                room: 1,
                sender: Default::default(),
                num: 1,
                unit: "月".into(),
                guard_level: 3,
                price: 138_000,
                timestamp: 1,
                archived: false,
            },
            cx,
        );
    });
    view
}

fn open<'a>(view: &Entity<GiftView>, cx: &'a mut TestAppContext) -> &'a mut VisualTestContext {
    let (_, window) = cx.add_window_view(|window, cx| Root::new(view.clone(), window, cx));
    window.simulate_resize(size(px(400.0), px(300.0)));
    window.run_until_parked();
    window
}

fn click(cx: &mut VisualTestContext, selector: &'static str) {
    let bounds = cx
        .debug_bounds(selector)
        .unwrap_or_else(|| panic!("{selector} not visible"));
    cx.simulate_click(bounds.center(), Modifiers::default());
    cx.run_until_parked();
}

#[gpui::test]
fn amount_filters_stay_live_when_collapsed_and_survive_a_new_host_window(cx: &mut TestAppContext) {
    let view = sample_records(cx);
    let window = open(&view, cx);
    assert!(window.debug_bounds("gift-min").is_none());
    click(window, "gift-filters");
    click(window, "gift-min");
    window.simulate_input("0.8");
    click(window, "gift-max");
    window.simulate_input("2");
    click(window, "gift-filters");
    assert!(window.debug_bounds("gift-min").is_none());
    assert!(
        window.debug_bounds("gift-archive-flower").is_some(),
        "range uses the total for both gifts"
    );
    assert!(window.debug_bounds("gift-archive-captain").is_none());

    window.update(|window, _| window.remove_window());
    let window = open(&view, cx);
    click(window, "gift-filters");
    click(window, "gift-max");
    window.simulate_keystrokes("backspace");
    assert!(
        window.debug_bounds("gift-archive-captain").is_some(),
        "an empty maximum removes the upper bound"
    );
    view.read_with(window, |view, _| assert_eq!(view.min_value, 0.8));
}

#[gpui::test]
fn archive_and_restore_are_single_click_actions(cx: &mut TestAppContext) {
    let view = sample_records(cx);
    let window = open(&view, cx);
    click(window, "gift-archive-captain");
    assert!(window.debug_bounds("gift-delete-captain").is_none());
    view.update(window, |view, cx| {
        view.show_archived = true;
        view.invalidate_cache();
        cx.notify();
    });
    window.run_until_parked();
    click(window, "gift-restore-captain");
    assert!(window.debug_bounds("gift-archive-captain").is_some());
    assert!(window.debug_bounds("gift-archive-flower").is_some());
}

#[gpui::test]
fn only_a_second_click_on_the_same_record_deletes_it(cx: &mut TestAppContext) {
    let view = sample_records(cx);
    let db = Arc::new(Database::in_memory().unwrap());
    view.update(cx, |view, cx| {
        for entry in &view.gift_list {
            match entry {
                GiftEntry::Gift(gift) => db.insert_gift(gift).unwrap(),
                GiftEntry::Guard(guard) => db.insert_guard(guard).unwrap(),
            }
        }
        view.load_from_database(&db, 1, cx);
    });
    let window = open(&view, cx);
    click(window, "gift-delete-flower");
    click(window, "gift-delete-captain");
    let first = window.debug_bounds("gift-delete-flower").unwrap();
    click(window, "gift-delete-flower");
    assert_eq!(
        window.debug_bounds("gift-confirm-delete-flower"),
        Some(first)
    );
    assert!(window.debug_bounds("gift-delete-captain").is_some());
    assert_eq!(db.get_recent_gifts(1, 2).unwrap().len(), 1);
    assert_eq!(db.get_recent_guards(1, 2).unwrap().len(), 1);

    click(window, "gift-confirm-delete-flower");
    assert!(db.get_recent_gifts(1, 2).unwrap().is_empty());
    assert_eq!(db.get_recent_guards(1, 2).unwrap().len(), 1);
    click(window, "gift-delete-captain");
    // A fast second click is delivered with click_count 2 by the OS.
    let position = window
        .debug_bounds("gift-confirm-delete-captain")
        .unwrap()
        .center();
    window.simulate_event(MouseDownEvent {
        button: MouseButton::Left,
        position,
        modifiers: Modifiers::default(),
        click_count: 2,
        first_mouse: false,
    });
    window.simulate_event(MouseUpEvent {
        button: MouseButton::Left,
        position,
        modifiers: Modifiers::default(),
        click_count: 2,
    });
    assert!(db.get_recent_guards(1, 2).unwrap().is_empty());
    view.read_with(window, |view, _| assert!(view.gift_list.is_empty()));
}

#[gpui::test]
fn interrupted_delete_requires_a_new_first_click(cx: &mut TestAppContext) {
    let view = sample_records(cx);
    let window = open(&view, cx);
    window.update(|window, _| window.activate_window());
    window.run_until_parked();
    click(window, "gift-delete-flower");
    click(window, "gift-filters");
    click(window, "gift-delete-flower");
    assert!(window.debug_bounds("gift-confirm-delete-flower").is_some());
    window.executor().advance_clock(Duration::from_secs(3));
    window.run_until_parked();
    click(window, "gift-delete-flower");
    let position = window
        .debug_bounds("gift-confirm-delete-flower")
        .unwrap()
        .center();
    window.simulate_event(ScrollWheelEvent {
        position,
        delta: ScrollDelta::Pixels(point(px(0.0), px(20.0))),
        ..Default::default()
    });
    click(window, "gift-delete-flower");
    window.deactivate_window();
    assert!(window.debug_bounds("gift-delete-flower").is_some());
    window.update(|window, _| window.activate_window());
    window.run_until_parked();
    click(window, "gift-delete-flower");
    window.update(|window, _| window.remove_window());
    let window = open(&view, cx);
    click(window, "gift-delete-flower");
    assert!(window.debug_bounds("gift-confirm-delete-flower").is_some());
    view.read_with(window, |view, _| assert_eq!(view.gift_list.len(), 2));
}

#[gpui::test]
fn clearing_from_more_requires_explicit_confirmation(cx: &mut TestAppContext) {
    let view = sample_records(cx);
    let window = open(&view, cx);
    for (answer, remaining) in [("取消", 2), ("清空", 0)] {
        click(window, "gift-more");
        window.simulate_keystrokes("up enter");
        assert!(window.has_pending_prompt());
        view.read_with(window, |v, _| assert_eq!(v.gift_list.len(), 2));
        window.simulate_prompt_answer(answer);
        window.run_until_parked();
        view.read_with(window, |v, _| assert_eq!(v.gift_list.len(), remaining));
    }
}

#[gpui::test]
fn online_history_from_more_requires_a_logged_in_account(cx: &mut TestAppContext) {
    let view = sample_records(cx);
    let (tx, rx) = std::sync::mpsc::channel();
    view.update(cx, |view, cx| view.set_query_command_sender(tx, cx));
    let window = open(&view, cx);
    click(window, "gift-more");
    window.simulate_keystrokes("down enter");
    view.read_with(window, |v, _| assert!(v.online_window.is_none()));
    assert!(rx.try_recv().is_err());

    view.update(window, |view, cx| {
        view.set_online_account(Some((42, "主播".into())), cx);
    });
    click(window, "gift-more");
    window.simulate_keystrokes("down enter");
    view.read_with(window, |v, _| assert!(v.online_window.is_some()));
    assert!(matches!(
        rx.try_recv(),
        Ok(super::UiCommand::FetchOnlineGiftTypes {
            account_uid: 42,
            ..
        })
    ));
}
