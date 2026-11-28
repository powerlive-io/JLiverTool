use super::{DisplayMessage, MainView, RenderRow};
use crate::app::UiCommand;
use gpui::{AppContext, Entity, Modifiers, TestAppContext, VisualTestContext};
use gpui_component::Root;
use jlivertool_core::events::Event;
use jlivertool_core::messages::{DanmuMessage, GiftMessage};
use serde_json::json;
use std::sync::mpsc;

fn gift(name: &str) -> Event {
    let body = json!({"data": {"uid": 1, "uname": "观众", "giftId": 1, "giftName": name}});
    Event::NewGift(GiftMessage::from_raw(&body, 1).unwrap())
}

fn visible_messages(main: &Entity<MainView>, cx: &TestAppContext) -> Vec<String> {
    cx.read(|cx| {
        main.read(cx)
            .render_rows
            .iter()
            .map(|row| match row {
                RenderRow::Full(DisplayMessage::Gift(gift)) => gift.gift_info.name.clone(),
                RenderRow::Full(DisplayMessage::Danmu(danmu)) => danmu.content.clone(),
                _ => panic!("expected a one-line gift or danmu"),
            })
            .collect()
    })
}

fn click(cx: &mut VisualTestContext, selector: &'static str) {
    let bounds = cx.debug_bounds(selector).expect(selector);
    cx.simulate_click(bounds.center(), Modifiers::default());
    cx.run_until_parked();
}

#[gpui::test]
fn gift_switch_hides_and_restores_the_feed_in_arrival_order(cx: &mut TestAppContext) {
    cx.update(gpui_component::init);
    let (events, rx) = mpsc::channel();
    let (tx, commands) = mpsc::channel();
    let main = cx.new(|cx| MainView::new(rx, tx, Default::default(), cx));
    events.send(gift("鲜花")).unwrap();
    let danmu = DanmuMessage::from_raw(&json!({"info": [[], "你好", [1, "观众"]]}), None).unwrap();
    events.send(Event::NewDanmu(danmu)).unwrap();

    cx.add_window(|window, cx| Root::new(main.clone(), window, cx));
    let settings = cx.read(|cx| main.read(cx).setting_view().clone());
    let (_, cx) = cx.add_window_view(|window, cx| Root::new(settings, window, cx));
    assert_eq!(visible_messages(&main, cx), ["鲜花", "你好"]);

    click(cx, "窗口设置");
    click(cx, "gift-display-switch");
    assert!(commands.try_iter().any(|cmd| matches!(
        cmd,
        UiCommand::UpdateWindowSettings {
            gift_display: false,
            ..
        }
    )));
    assert_eq!(visible_messages(&main, cx), ["你好"]);

    events.send(gift("星星")).unwrap();
    main.update(cx, |_, cx| cx.notify());
    cx.run_until_parked();
    assert_eq!(visible_messages(&main, cx), ["你好"]);

    click(cx, "gift-display-switch");
    assert_eq!(visible_messages(&main, cx), ["鲜花", "你好", "星星"]);
}
