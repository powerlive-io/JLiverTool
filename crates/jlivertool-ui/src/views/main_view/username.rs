//! Pronunciation tooltips for names in the danmu feed.

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::tooltip::Tooltip;
use pinyin::ToPinyin;

fn pronunciation(name: &str) -> Option<String> {
    let mut parts = Vec::new();
    let mut start = 0;
    for (i, ch) in name.char_indices() {
        if let Some(reading) = ch.to_pinyin() {
            let plain = name[start..i].trim();
            if !plain.is_empty() {
                parts.push(plain);
            }
            parts.push(reading.with_tone());
            start = i + ch.len_utf8();
        }
    }
    if parts.is_empty() {
        return None;
    }
    let rest = name[start..].trim();
    if !rest.is_empty() {
        parts.push(rest);
    }
    Some(parts.join(" "))
}

/// `display_text` may include punctuation or just one wrapped fragment. The
/// tooltip always pronounces the original name, and only after the hover delay.
pub(super) fn username_label(
    id: impl Into<ElementId>,
    name: &str,
    display_text: Option<String>,
    font_size: f32,
) -> Stateful<Div> {
    let id = id.into();
    let name = SharedString::new(name);
    let text = display_text
        .map(SharedString::from)
        .unwrap_or_else(|| name.clone());
    let label = div()
        .text_size(px(font_size))
        .child(text)
        .when(cfg!(test), |el| el.debug_selector(|| id.to_string()));
    if name.is_ascii() {
        return label.id(id);
    }

    // A virtual list can reuse this row index for another user while hovered.
    label
        .id((id, name.clone()))
        .tooltip(move |window, cx| match pronunciation(&name) {
            Some(reading) => Tooltip::new(reading)
                .max_w(window.viewport_size().width * 0.8)
                .build(window, cx),
            None => cx.new(|_| EmptyView).into(),
        })
}

#[cfg(test)]
mod tests {
    use super::pronunciation;
    use crate::{views::MainView, UiCommand};
    use gpui::{AppContext, Modifiers, TestAppContext};
    use gpui_component::{tooltip::Tooltip, Root};
    use jlivertool_core::{events::Event, messages::DanmuMessage, types::Sender};
    use std::{
        cell::Cell,
        rc::Rc,
        sync::{atomic::AtomicBool, mpsc, Arc},
        time::Duration,
    };

    #[test]
    fn reads_hanzi_and_keeps_mixed_name_segments() {
        assert_eq!(pronunciation("林小满").as_deref(), Some("lín xiǎo mǎn"));
        assert_eq!(
            pronunciation("Mo_云朵☁️27").as_deref(),
            Some("Mo_ yún duǒ ☁️27")
        );
        for name in ["", "Nova_27", "🐾", "éclair"] {
            assert_eq!(pronunciation(name), None);
        }
    }

    #[gpui::test]
    fn hovering_a_name_shows_a_tip_without_reflow_or_intercepting_clicks(cx: &mut TestAppContext) {
        cx.update(gpui_component::init);
        let tips = Rc::new(Cell::new(0));
        let _observe = cx.update(|cx| {
            cx.observe_new::<Tooltip>({
                let tips = tips.clone();
                move |_, _, _| tips.set(tips.get() + 1)
            })
        });

        let (tx, events) = mpsc::channel();
        let (commands, rx) = mpsc::channel();
        for (uid, name) in [(42, "Cat_双双9🐾"), (43, "Nova_27")] {
            tx.send(Event::NewDanmu(DanmuMessage {
                sender: Sender {
                    uid,
                    uname: name.into(),
                    ..Default::default()
                },
                content: "晚上好".into(),
                is_generated: false,
                is_special: false,
                is_mirror: false,
                emoji_content: None,
                side_index: -1,
                reply_uname: None,
            }))
            .unwrap();
        }
        let (_, feed) = cx.add_window_view(|window, cx| {
            let main =
                cx.new(|cx| MainView::new(events, commands, Arc::new(AtomicBool::new(false)), cx));
            Root::new(main, window, cx)
        });
        let name = feed.debug_bounds("user-0").unwrap();
        let next = feed.debug_bounds("user-1").unwrap();
        feed.simulate_mouse_move(name.center(), None, Modifiers::default());
        assert_eq!(
            tips.get(),
            0,
            "no tooltip is built during ordinary rendering"
        );
        feed.dispatcher.advance_clock(Duration::from_secs(1));
        feed.run_until_parked();
        assert_eq!(tips.get(), 1);
        assert_eq!(feed.debug_bounds("user-0"), Some(name));
        assert_eq!(feed.debug_bounds("user-1"), Some(next));

        feed.simulate_mouse_move(next.center(), None, Modifiers::default());
        feed.dispatcher.advance_clock(Duration::from_secs(1));
        feed.run_until_parked();
        assert_eq!(tips.get(), 1, "an ASCII name needs no tip");
        feed.simulate_mouse_move(name.center(), None, Modifiers::default());
        feed.dispatcher.advance_clock(Duration::from_secs(1));
        feed.run_until_parked();
        assert_eq!(tips.get(), 2, "re-entering the name opens a fresh tip");
        feed.simulate_click(name.center(), Modifiers::default());
        assert!(rx
            .try_iter()
            .any(|cmd| matches!(cmd, UiCommand::FetchUserInfo { uid: 42 })));
    }
}
