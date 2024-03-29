import { BaseDirectory, createDir, writeFile, readTextFile } from "@tauri-apps/api/fs";

async function get(key, defaultValue) {
    try {
        const data = JSON.parse(await readDataFile());
        
        if (data) return data[key] ?? defaultValue;
    } catch(e) {
        return defaultValue;
    }

    return defaultValue;
}

async function set(key, value) {
    createDataFolder();
    try {
        const data = JSON.parse(await readDataFile().catch(() => "{}"));
        
        data[key] = value;
        setFileData({...data});
    } catch(e) {
        setFileData({
            [key]: value
        });
    }
}

// So we can use names like store.get and store.set, which looks cleaner.
export default { get, set }

const createDataFolder = async () => {
    await createDir("CodeMacros", {
        dir: BaseDirectory.Config,
        recursive: true,
    })
};

const setFileData = async (data) => {
    return writeFile(
        {
            contents: JSON.stringify(data),
            path: `CodeMacros/config.json`,
        },
        {
            dir: BaseDirectory.Config,
        }
    );
};

const readDataFile = () => {
    return readTextFile("CodeMacros/config.json", {
        dir: BaseDirectory.Config,
    })
}