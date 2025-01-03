use lazy_static::lazy_static;
use std::collections::HashMap;

//const KEYCODE_DISPLAY_NAMES: &'static [(&str, &str)] = &[
fn init_hashmap() -> HashMap<&'static str, &'static str> {
    let keycode_display_names = [
        ("KC_C", "C"),
        ("KC_D", "D"),
        ("KC_E", "E"),
        ("KC_F", "F"),
        ("KC_G", "G"),
        ("KC_H", "H"),
        ("KC_I", "I"),
        ("KC_J", "J"),
        ("KC_K", "K"),
        ("KC_L", "L"),
        ("KC_M", "M"),
        ("KC_N", "N"),
        ("KC_O", "O"),
        ("KC_P", "P"),
        ("KC_Q", "Q"),
        ("KC_R", "R"),
        ("KC_S", "S"),
        ("KC_T", "T"),
        ("KC_U", "U"),
        ("KC_V", "V"),
        ("KC_W", "W"),
        ("KC_X", "X"),
        ("KC_Y", "Y"),
        ("KC_Z", "Z"),
        ("KC_1", "1"),
        ("KC_2", "2"),
        ("KC_3", "3"),
        ("KC_4", "4"),
        ("KC_5", "5"),
        ("KC_6", "6"),
        ("KC_7", "7"),
        ("KC_8", "8"),
        ("KC_9", "9"),
        ("KC_0", "0"),
        ("KC_ENTER", ""),
        ("KC_ESCAPE", ""),
        ("KC_BACKSPACE", ""),
        ("KC_TAB", ""),
        ("KC_SPACE", " "),
        ("KC_MINUS", "-"),
        ("KC_EQUAL", "="),
        ("KC_LEFT_BRACKET", "["),
        ("KC_RIGHT_BRACKET", "]"),
        ("KC_BACKSLASH", "\\"),
        ("KC_NONUS_HASH", "#"),
        ("KC_SEMICOLON", ";"),
        ("KC_QUOTE", "'"),
        ("KC_GRAVE", "`"),
        ("KC_COMMA", ","),
        ("KC_DOT", "."),
        ("KC_SLASH", "/"),
        ("KC_CAPS_LOCK", ""),
        ("KC_F1", "F1"),
        ("KC_F2", "F2"),
        ("KC_F3", "F3"),
        ("KC_F4", "F4"),
        ("KC_F5", "F5"),
        ("KC_F6", "F6"),
        ("KC_F7", "F7"),
        ("KC_F8", "F8"),
        ("KC_F9", "F9"),
        ("KC_F10", "F10"),
        ("KC_F11", "F11"),
        ("KC_F12", "F12"),
        ("KC_PRINT_SCREEN", ""),
        ("KC_SCROLL_LOCK", ""),
        ("KC_PAUSE", ""),
        ("KC_INSERT", ""),
        ("KC_HOME", ""),
        ("KC_PAGE_UP", ""),
        ("KC_DELETE", ""),
        ("KC_END", ""),
        ("KC_PAGE_DOWN", ""),
        ("KC_RIGHT", ""),
        ("KC_LEFT", ""),
        ("KC_DOWN", ""),
        ("KC_UP", ""),
        ("KC_NUM_LOCK", ""),
        ("KC_KP_SLASH", "/"),
        ("KC_KP_ASTERISK", "*"),
        ("KC_KP_MINUS", "-"),
        ("KC_KP_PLUS", "+"),
        ("KC_KP_ENTER", ""),
        ("KC_KP_1", "1"),
        ("KC_KP_2", "2"),
        ("KC_KP_3", "3"),
        ("KC_KP_4", "4"),
        ("KC_KP_5", "5"),
        ("KC_KP_6", "6"),
        ("KC_KP_7", "7"),
        ("KC_KP_8", "8"),
        ("KC_KP_9", "9"),
        ("KC_KP_0", "0"),
        ("KC_KP_DOT", "."),
        ("KC_NONUS_BACKSLASH", "\\"),
        ("KC_APPLICATION", ""),
        ("KC_KB_POWER", ""),
        ("KC_KP_EQUAL", "="),
        ("KC_F13", ""),
        ("KC_F14", ""),
        ("KC_F15", ""),
        ("KC_F16", ""),
        ("KC_F17", ""),
        ("KC_F18", ""),
        ("KC_F19", ""),
        ("KC_F20", ""),
        ("KC_F21", ""),
        ("KC_F22", ""),
        ("KC_F23", ""),
        ("KC_F24", ""),
        ("KC_EXECUTE", ""),
        ("KC_HELP", ""),
        ("KC_MENU", ""),
        ("KC_SELECT", ""),
        ("KC_STOP", ""),
        ("KC_AGAIN", ""),
        ("KC_UNDO", ""),
        ("KC_CUT", ""),
        ("KC_COPY", ""),
        ("KC_PASTE", ""),
        ("KC_FIND", ""),
        ("KC_KB_MUTE", ""),
        ("KC_KB_VOLUME_UP", ""),
        ("KC_KB_VOLUME_DOWN", ""),
        ("KC_LOCKING_CAPS_LOCK", ""),
        ("KC_LOCKING_NUM_LOCK", ""),
        ("KC_LOCKING_SCROLL_LOCK", ""),
        ("KC_KP_COMMA", ""),
        ("KC_KP_EQUAL_AS400", ""),
        ("KC_INTERNATIONAL_1", ""),
        ("KC_INTERNATIONAL_2", ""),
        ("KC_INTERNATIONAL_3", ""),
        ("KC_INTERNATIONAL_4", ""),
        ("KC_INTERNATIONAL_5", ""),
        ("KC_INTERNATIONAL_6", ""),
        ("KC_INTERNATIONAL_7", ""),
        ("KC_INTERNATIONAL_8", ""),
        ("KC_INTERNATIONAL_9", ""),
        ("KC_LANGUAGE_1", ""),
        ("KC_LANGUAGE_2", ""),
        ("KC_LANGUAGE_3", ""),
        ("KC_LANGUAGE_4", ""),
        ("KC_LANGUAGE_5", ""),
        ("KC_LANGUAGE_6", ""),
        ("KC_LANGUAGE_7", ""),
        ("KC_LANGUAGE_8", ""),
        ("KC_LANGUAGE_9", ""),
        ("KC_ALTERNATE_ERASE", ""),
        ("KC_SYSTEM_REQUEST", ""),
        ("KC_CANCEL", ""),
        ("KC_CLEAR", ""),
        ("KC_PRIOR", ""),
        ("KC_RETURN", ""),
        ("KC_SEPARATOR", ""),
        ("KC_OUT", ""),
        ("KC_OPER", ""),
        ("KC_CLEAR_AGAIN", ""),
        ("KC_CRSEL", ""),
        ("KC_EXSEL", ""),
        ("KC_SYSTEM_POWER", ""),
        ("KC_SYSTEM_SLEEP", ""),
        ("KC_SYSTEM_WAKE", ""),
        ("KC_AUDIO_MUTE", ""),
        ("KC_AUDIO_VOL_UP", ""),
        ("KC_AUDIO_VOL_DOWN", ""),
        ("KC_MEDIA_NEXT_TRACK", ""),
        ("KC_MEDIA_PREV_TRACK", ""),
        ("KC_MEDIA_STOP", ""),
        ("KC_MEDIA_PLAY_PAUSE", ""),
        ("KC_MEDIA_SELECT", ""),
        ("KC_MEDIA_EJECT", ""),
        ("KC_MAIL", ""),
        ("KC_CALCULATOR", ""),
        ("KC_MY_COMPUTER", ""),
        ("KC_WWW_SEARCH", ""),
        ("KC_WWW_HOME", ""),
        ("KC_WWW_BACK", ""),
        ("KC_WWW_FORWARD", ""),
        ("KC_WWW_STOP", ""),
        ("KC_WWW_REFRESH", ""),
        ("KC_WWW_FAVORITES", ""),
        ("KC_MEDIA_FAST_FORWARD", ""),
        ("KC_MEDIA_REWIND", ""),
        ("KC_BRIGHTNESS_UP", ""),
        ("KC_BRIGHTNESS_DOWN", ""),
        ("KC_CONTROL_PANEL", ""),
        ("KC_ASSISTANT", ""),
        ("KC_MISSION_CONTROL", ""),
        ("KC_LAUNCHPAD", ""),
    ];
    HashMap::from(keycode_display_names)
}

pub fn display_name(keycode: &str) -> Option<&'static str> {
    lazy_static! {
        static ref display_names: HashMap<&'static str, &'static str> = init_hashmap();
    }
    display_names.get(keycode).map(|v| &**v)
}
