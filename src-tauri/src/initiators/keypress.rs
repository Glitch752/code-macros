use inputbot::{ KeybdKey };

use std::collections::HashMap;
use std::sync::{ Arc, Mutex };

use super::super::get_macros;

use super::super::execute::run_macro_initiator;

pub fn listen_initiator_keypress() {
    let keys_pressed: Arc<Mutex<HashMap<KeybdKey, bool>>> = Arc::new(Mutex::new(HashMap::new()));
    KeybdKey::bind_all(move |event| {

        let mut keys_pressed = keys_pressed.lock().unwrap();

        // Turning the value from true to false is a hacky workaround for windows
        if !keys_pressed.contains_key(&event) {
            keys_pressed.insert(event, true);
        }

        // Loop through all keys and check if they are pressed. If not, remove them from the map.
        let mut remove = Vec::new();
        for key in keys_pressed.clone().keys() {
            if !KeybdKey::is_pressed(*key) {
                if keys_pressed[key] == false {
                    remove.push(*key);
                } else {
                    *keys_pressed.get_mut(&event).unwrap() = false;
                }
            }
        }
        for key in remove {
            keys_pressed.remove(&key);
        }

        let mut keys_pressed_js: Vec<String> = vec![];
        for key in keys_pressed.keys() {
            keys_pressed_js.push(js_key(*key));
        }

        'macros: for macro_ in get_macros() {
            // Check if macro_.macro_.initiators is Some
            if macro_.macro_.initiators.is_some() {
                let initiators = macro_.macro_.initiators.as_ref().unwrap();
                // Check if the initiators are pressed
                for initiator in initiators {
                    if initiator.type_ == "keypress" {
                        let keys = initiator.data.keys.as_ref().unwrap();
                        for key in keys {
                            if !keys_pressed_js.contains(key) {
                                continue 'macros;
                            }
                        }
                        run_macro_initiator(initiator.clone(), macro_.clone());
                    }
                }
            }
        }
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}

// Turn the key enum into the same format as comes from the macro config
fn js_key(key: KeybdKey) -> String {
    let mut enum_to_key:HashMap<String, String> = HashMap::new();
    enum_to_key.insert("AKey".to_string(),         "a".to_string()         );
    enum_to_key.insert("BKey".to_string(),         "b".to_string()         );
    enum_to_key.insert("CKey".to_string(),         "c".to_string()         );
    enum_to_key.insert("DKey".to_string(),         "d".to_string()         );
    enum_to_key.insert("EKey".to_string(),         "e".to_string()         );
    enum_to_key.insert("FKey".to_string(),         "f".to_string()         );
    enum_to_key.insert("GKey".to_string(),         "g".to_string()         );
    enum_to_key.insert("HKey".to_string(),         "h".to_string()         );
    enum_to_key.insert("IKey".to_string(),         "i".to_string()         );
    enum_to_key.insert("JKey".to_string(),         "j".to_string()         );
    enum_to_key.insert("KKey".to_string(),         "k".to_string()         );
    enum_to_key.insert("LKey".to_string(),         "l".to_string()         );
    enum_to_key.insert("MKey".to_string(),         "m".to_string()         );
    enum_to_key.insert("NKey".to_string(),         "n".to_string()         );
    enum_to_key.insert("OKey".to_string(),         "o".to_string()         );
    enum_to_key.insert("PKey".to_string(),         "p".to_string()         );
    enum_to_key.insert("QKey".to_string(),         "q".to_string()         );
    enum_to_key.insert("RKey".to_string(),         "r".to_string()         );
    enum_to_key.insert("SKey".to_string(),         "s".to_string()         );
    enum_to_key.insert("TKey".to_string(),         "t".to_string()         );
    enum_to_key.insert("UKey".to_string(),         "u".to_string()         );
    enum_to_key.insert("VKey".to_string(),         "v".to_string()         );
    enum_to_key.insert("WKey".to_string(),         "w".to_string()         );
    enum_to_key.insert("XKey".to_string(),         "x".to_string()         );
    enum_to_key.insert("YKey".to_string(),         "y".to_string()         );
    enum_to_key.insert("ZKey".to_string(),         "z".to_string()         );
    enum_to_key.insert("Numrow0Key".to_string(),   "0".to_string()         );
    enum_to_key.insert("Numrow1Key".to_string(),   "1".to_string()         );
    enum_to_key.insert("Numrow2Key".to_string(),   "2".to_string()         );
    enum_to_key.insert("Numrow3Key".to_string(),   "3".to_string()         );
    enum_to_key.insert("Numrow4Key".to_string(),   "4".to_string()         );
    enum_to_key.insert("Numrow5Key".to_string(),   "5".to_string()         );
    enum_to_key.insert("Numrow6Key".to_string(),   "6".to_string()         );
    enum_to_key.insert("Numrow7Key".to_string(),   "7".to_string()         );
    enum_to_key.insert("Numrow8Key".to_string(),   "8".to_string()         );
    enum_to_key.insert("Numrow9Key".to_string(),   "9".to_string()         );
    enum_to_key.insert("Numpad0Key".to_string(),   "0".to_string()         );
    enum_to_key.insert("Numpad1Key".to_string(),   "1".to_string()         );
    enum_to_key.insert("Numpad2Key".to_string(),   "2".to_string()         );
    enum_to_key.insert("Numpad3Key".to_string(),   "3".to_string()         );
    enum_to_key.insert("Numpad4Key".to_string(),   "4".to_string()         );
    enum_to_key.insert("Numpad5Key".to_string(),   "5".to_string()         );
    enum_to_key.insert("Numpad6Key".to_string(),   "6".to_string()         );
    enum_to_key.insert("Numpad7Key".to_string(),   "7".to_string()         );
    enum_to_key.insert("Numpad8Key".to_string(),   "8".to_string()         );
    enum_to_key.insert("Numpad9Key".to_string(),   "9".to_string()         );
    enum_to_key.insert("BackspaceKey".to_string(), "backspace".to_string() );
    enum_to_key.insert("TabKey".to_string(),       "tab".to_string()       );
    enum_to_key.insert("EnterKey".to_string(),     "enter".to_string()     );
    enum_to_key.insert("EscapeKey".to_string(),    "escape".to_string()    );
    enum_to_key.insert("SpaceKey".to_string(),     "space".to_string()     );
    enum_to_key.insert("F1Key".to_string(),        "f1".to_string()        );
    enum_to_key.insert("F2Key".to_string(),        "f2".to_string()        );
    enum_to_key.insert("F3Key".to_string(),        "f3".to_string()        );
    enum_to_key.insert("F4Key".to_string(),        "f4".to_string()        );
    enum_to_key.insert("F5Key".to_string(),        "f5".to_string()        );
    enum_to_key.insert("F6Key".to_string(),        "f6".to_string()        );
    enum_to_key.insert("F7Key".to_string(),        "f7".to_string()        );
    enum_to_key.insert("F8Key".to_string(),        "f8".to_string()        );
    enum_to_key.insert("F9Key".to_string(),        "f9".to_string()        );
    enum_to_key.insert("F10Key".to_string(),       "f10".to_string()       );
    enum_to_key.insert("CapsLockKey".to_string(),  "capslock".to_string()  );
    enum_to_key.insert("QuoteKey".to_string(),     "'".to_string()         );
    enum_to_key.insert("SemicolonKey".to_string(), ";".to_string()         );
    enum_to_key.insert("CommaKey".to_string(),     ",".to_string()         );
    enum_to_key.insert("PeriodKey".to_string(),    ".".to_string()         );
    enum_to_key.insert("SlashKey".to_string(),     "/".to_string()         );
    enum_to_key.insert("BackslashKey".to_string(), "\\".to_string()        );
    enum_to_key.insert("MinusKey".to_string(),     "-".to_string()         );
    enum_to_key.insert("EqualKey".to_string(),     "=".to_string()         );
    enum_to_key.insert("LBracketKey".to_string(),  "[".to_string()         );
    enum_to_key.insert("RBracketKey".to_string(),  "]".to_string()         );
    enum_to_key.insert("BackquoteKey".to_string(), "`".to_string()         );
    enum_to_key.insert("LShiftKey".to_string(),    "shift".to_string()     );
    enum_to_key.insert("RShiftKey".to_string(),    "shift".to_string()     );
    enum_to_key.insert("LControlKey".to_string(),  "control".to_string()   );
    enum_to_key.insert("RControlKey".to_string(),  "control".to_string()   );
    // Does not work on linux
    enum_to_key.insert("RightKey".to_string(),     "right".to_string()     );
    enum_to_key.insert("LeftKey".to_string(),      "left".to_string()      );
    enum_to_key.insert("UpKey".to_string(),        "up".to_string()        );
    enum_to_key.insert("DownKey".to_string(),      "down".to_string()      );
    enum_to_key.insert("F11Key".to_string(),       "f11".to_string()       );
    enum_to_key.insert("F12Key".to_string(),       "f12".to_string()       );

    let key_string = format!("{:?}", key);

    return enum_to_key.get(&key_string).unwrap().to_string();
}