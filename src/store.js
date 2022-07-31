export function get(key, defaultValue) {

}

export function set(key, value) {
    createDataFolder();
}

import { BaseDirectory, createDir } from "@tauri-apps/api/fs";

const createDataFolder = async () => {
    console.log("ran");

    await createDir("codeMacros", {
        dir: BaseDirectory.Config,
        recursive: true,
    });
};

const createDataFile = async () => {
    try {
        await writeFile(
            {
                contents: "[]",
                path: `./data/data.json`,
            },
            {
                dir: dir,
            }
        );
    } catch (e) {
        console.log(e);
    }
};