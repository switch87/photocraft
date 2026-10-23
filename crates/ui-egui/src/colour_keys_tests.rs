//! X swaps the foreground and background colours and D resets them, as in Photoshop, after a
//! click on the canvas too (#8619): only a text field with keyboard focus keeps those letters.

use egui::{Key, vec2};
use egui_kittest::Harness;
use serde_json::json;

use crate::PhotocraftApp;
use crate::state::Tool;

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

fn harness(tool: Tool) -> Harness<'static, PhotocraftApp> {
    let mut h = Harness::builder().with_size(vec2(1440.0, 900.0)).with_max_steps(64).build_eframe(|cc| {
        PhotocraftApp::setup_context(&cc.egui_ctx, Default::default());
        PhotocraftApp::new(photocraft_engine::Session::new(), crate::Services::default())
    });
    h.state_mut().run("file.new", json!({"width": 400, "height": 300})).unwrap();
    h.state_mut().ui.tool = tool;
    h.run_steps(4);
    set_colours(&mut h);
    h
}

fn set_colours(h: &mut Harness<'_, PhotocraftApp>) {
    let tools = &mut h.state_mut().session.tools;
    tools.foreground = RED;
    tools.background = BLUE;
    h.run_steps(2);
}

fn colours(h: &Harness<'_, PhotocraftApp>) -> ([f32; 4], [f32; 4]) {
    let tools = &h.state().session.tools;
    (tools.foreground, tools.background)
}

/// A real press and release of the primary button on the canvas centre.
fn click_canvas(h: &mut Harness<'_, PhotocraftApp>) {
    let p = h.state().last_canvas_rect.center();
    h.event(egui::Event::PointerMoved(p));
    h.run_steps(2);
    for pressed in [true, false] {
        h.event(egui::Event::PointerButton { pos: p, button: egui::PointerButton::Primary, pressed, modifiers: egui::Modifiers::NONE });
        h.run_steps(2);
    }
    h.run_steps(2);
}

fn press(h: &mut Harness<'_, PhotocraftApp>, key: Key) {
    h.key_press(key);
    h.run_steps(3);
}

#[test]
fn x_swaps_the_colours_after_a_click_on_the_canvas() {
    for tool in [Tool::Move, Tool::RectMarquee, Tool::Brush, Tool::Hand] {
        let mut h = harness(tool);
        press(&mut h, Key::X);
        assert_eq!(colours(&h), (BLUE, RED), "{tool:?}: X before any click");
        press(&mut h, Key::X);
        click_canvas(&mut h);
        set_colours(&mut h);
        press(&mut h, Key::X);
        assert_eq!(colours(&h), (BLUE, RED), "{tool:?}: X after a click on the canvas");
        press(&mut h, Key::X);
        assert_eq!(colours(&h), (RED, BLUE), "{tool:?}: and again");
    }
}

#[test]
fn d_resets_the_colours_after_a_click_on_the_canvas() {
    for tool in [Tool::Move, Tool::RectMarquee, Tool::Brush, Tool::Hand] {
        let mut h = harness(tool);
        click_canvas(&mut h);
        set_colours(&mut h);
        press(&mut h, Key::D);
        let (fg, bg) = colours(&h);
        assert_ne!((fg, bg), (RED, BLUE), "{tool:?}: D after a click on the canvas resets the colours");
        assert_eq!(fg[..3], [0.0, 0.0, 0.0], "{tool:?}: black foreground");
        assert_eq!(bg[..3], [1.0, 1.0, 1.0], "{tool:?}: white background");
    }
}
