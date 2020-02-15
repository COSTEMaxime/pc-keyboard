//! ANSI 103-key fr keyboard.
//! Has a 1-row high Enter key, with '*' above.

use crate::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

pub use super::us104::Us104Key;

pub struct ANSI103fr;

impl KeyboardLayout for ANSI103fr {
    fn map_keycode(
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        let map_to_unicode = handle_ctrl == HandleControl::MapLettersToUnicode;
        match keycode {
            KeyCode::BackTick => DecodedKey::Unicode('²'),
            KeyCode::Key1 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('1')
                } else {
                    DecodedKey::Unicode('&')
                }
            }
            KeyCode::Key2 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('2')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('~')
                } else {
                    DecodedKey::Unicode('é')
                }
            }
            KeyCode::Key3 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('3')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('#')
                } else {
                    DecodedKey::Unicode('"')
                }
            }
            KeyCode::Key4 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('4')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('{')
                } else {
                    DecodedKey::Unicode('\'')
                }
            }
            KeyCode::Key5 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('5')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('[')
                } else {
                    DecodedKey::Unicode('(')
                }
            }
            KeyCode::Key6 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('6')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('|')
                } else {
                    DecodedKey::Unicode('-')
                }
            }
            KeyCode::Key7 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('7')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('`')
                } else {
                    DecodedKey::Unicode('è')
                }
            }
            KeyCode::Key8 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('8')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('\\')
                } else {
                    DecodedKey::Unicode('_')
                }
            }
            KeyCode::Key9 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('9')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('^')
                } else {
                    DecodedKey::Unicode('ç')
                }
            }
            KeyCode::Key0 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('0')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('@')
                } else {
                    DecodedKey::Unicode('à')
                }
            }
            KeyCode::Minus => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('°')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode(']')
                } else {
                    DecodedKey::Unicode(')')
                }
            }
            KeyCode::Equals => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('+')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('}')
                } else {
                    DecodedKey::Unicode('=')
                }
            }
            KeyCode::A => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0011}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Q')
                } else {
                    DecodedKey::Unicode('q')
                }
            }
            KeyCode::Z => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0017}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('W')
                } else {
                    DecodedKey::Unicode('w')
                }
            }
            KeyCode::BracketSquareLeft => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('¨')
                } else {
                    DecodedKey::Unicode('^')
                }
            }
            KeyCode::BracketSquareRight => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('£')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('¤')
                } else {
                    DecodedKey::Unicode('$')
                }
            }
            KeyCode::BackSlash => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('µ')
                } else {
                    DecodedKey::Unicode('*')
                }
            }
            KeyCode::Q => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0001}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('A')
                } else {
                    DecodedKey::Unicode('a')
                }
            }
            KeyCode::SemiColon => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000D}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('M')
                } else {
                    DecodedKey::Unicode('m')
                }
            }
            KeyCode::Quote => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('%')
                } else {
                    DecodedKey::Unicode('ù')
                }
            }
            KeyCode::W => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{001A}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Z')
                } else {
                    DecodedKey::Unicode('z')
                }
            }
            KeyCode::M => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('?')
                } else {
                    DecodedKey::Unicode(',')
                }
            }
            KeyCode::Comma => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('.')
                } else {
                    DecodedKey::Unicode(';')
                }
            }
            KeyCode::Fullstop => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('/')
                } else {
                    DecodedKey::Unicode(':')
                }
            }
            KeyCode::Slash => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('§')
                } else {
                    DecodedKey::Unicode('!')
                }
            }
            e => <super::Us104Key as KeyboardLayout>::map_keycode(e, modifiers, handle_ctrl),
        }
    }
}