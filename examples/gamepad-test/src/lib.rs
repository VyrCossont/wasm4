mod wasm4;
use wasm4::*;

/// WASM-4 `text()` actually expects ASCII with some nonstandard
/// high extensions for gamepad buttons, not UTF-8.
fn btext(t: &[u8], x: i32, y: i32) {
    let extd_ascii_text = unsafe {
        std::str::from_utf8_unchecked(t)
    };
    text(extd_ascii_text, x, y);
}

fn set_color_for_button(gamepad_state: u8, button_mask: u8) {
    let color: u16 = if gamepad_state & button_mask != 0 {
        4
    } else {
        2
    };
    unsafe { *DRAW_COLORS = color }
}

fn draw_gamepad(label: &[u8], gamepad: *const u8, x: i32, y: i32) {
    unsafe { *DRAW_COLORS = 2 }
    btext(label, x + 10, y + 10);

    unsafe { *DRAW_COLORS = 0x20 }
    rect(x, y, 80, 80);

    let gamepad_state = unsafe { *gamepad };

    set_color_for_button(gamepad_state, BUTTON_1);
    btext(b"\x80", x + 60, y + 40);
    
    set_color_for_button(gamepad_state, BUTTON_2);
    btext(b"\x81", x + 50, y + 40);
    
    set_color_for_button(gamepad_state, BUTTON_LEFT);
    btext(b"\x84", x + 10, y + 40);
    
    set_color_for_button(gamepad_state, BUTTON_RIGHT);
    btext(b"\x85", x + 30, y + 40);
    
    set_color_for_button(gamepad_state, BUTTON_UP);
    btext(b"\x86", x + 20, y + 30);
    
    set_color_for_button(gamepad_state, BUTTON_DOWN);
    btext(b"\x87", x + 20, y + 50);
}

#[no_mangle]
fn update() {
    draw_gamepad(b"Player 1", GAMEPAD1, 0, 0);
    draw_gamepad(b"Player 2", GAMEPAD2, 80, 0);
    draw_gamepad(b"Player 3", GAMEPAD3, 0, 80);
    draw_gamepad(b"Player 4", GAMEPAD4, 80, 80);
}
