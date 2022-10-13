import { invoke } from '@tauri-apps/api/tauri';
import * as store from './store';

export default function updateMacros() {
    store.get('macros', []).then((data) => {
        invoke('update_macros', { macros: data
            .map(object => {
                return ensureKeys(recursivelyReplaceKeys(
                    object,
                    [
                        { from: 'macro', to: 'macro_' },
                        { from: 'type', to: 'type_' },
                        { from: 'loop', to: 'loop_' },
                        { from: 'else', to: 'else_' },
                        { from: 'codeInside', to: 'code_inside' },
                        { from: 'activateTime', to: 'activate_time' },
                        { from: 'appPath', to: 'app_path' },
                    ]
                ), ["macro_", "name", "description"]);
            }).filter(object => object !== null)
        });
    });
}

function recursivelyReplaceKeys(object, replacements) {
    if(object instanceof Array) {
        return object.map(item => recursivelyReplaceKeys(item, replacements));
    }
    
    if(object instanceof Object) {
        return Object.fromEntries(Object.entries(object).map(([key, value]) => {
            for(const replacement of replacements) {
                if(key === replacement.from) {
                    return [replacement.to, value];
                }
            }
            return [key, value];
        }).map(([key, value]) => [key, typeof value === "object" ? recursivelyReplaceKeys(value, replacements) : value]));
    }
    
    return object;
}

function ensureKeys(object, keys) {
    // Remove all keys that are not in the keys array and return null if there are keys not in the keys array.
    let result = {};
    for(const key of keys) {
        if(object.hasOwnProperty(key)) {
            result[key] = object[key];
        } else {
            return null;
        }
    }
    return result;
}

export function selectTheme(e) {
    loadTheme(e.target.value);
    store.set("theme", e.target.value);
}

export function loadTheme(theme = null) {
    if(theme === null) {
        store.get("theme", "darkTheme").then((theme) => {
            loadTheme(theme);
        });
    } else {
        document.body.className = "";
        document.body.classList.add(theme);

        document.themeChangedCallback();
    }
}