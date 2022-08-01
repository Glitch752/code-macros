import { BaseDirectory, createDir, writeFile, readTextFile } from "@tauri-apps/api/fs";

export async function get(key, defaultValue) {
    return new Promise(async (resolve, reject) => {
        let data;
        try {
            data = JSON.parse(await readDataFile());
        } catch(e) {} finally {
            if (data) {
                resolve(data[key] || defaultValue);
            } else {
                resolve(defaultValue);
            }
        }
    });
}

export async function set(key, value) {
    createDataFolder();
    let data;
    try {
        data = JSON.parse(await readDataFile().catch(() => "{}"));
    } catch(e) {} finally {
        if(!data) data = {};
        data[key] = value;
        setFileData({...data});
    }
}

const createDataFolder = async () => {
    await createDir("codeMacros", {
        dir: BaseDirectory.Config,
        recursive: true,
    })
};

const setFileData = async (data) => {
    return writeFile(
        {
            contents: JSON.stringify(data),
            path: `codeMacros/config.json`,
        },
        {
            dir: BaseDirectory.Config,
        }
    );
};

const readDataFile = () => {
    return readTextFile("codeMacros/config.json", {
        dir: BaseDirectory.Config,
    })
}