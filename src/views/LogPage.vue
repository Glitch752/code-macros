<script setup>
    import { BaseDirectory, createDir, writeFile, readTextFile } from "@tauri-apps/api/fs";
    import { ref } from "vue";

    async function getLog() {
        return new Promise(async (resolve, reject) => {
            let data;
            try {
                data = await readDataFile();
            } catch(e) {
                console.error(e);
            } finally {
                if (data) {
                    resolve(data);
                } else {
                    resolve("No log data");
                }
            }
        });
    }

    // async function setLog(key, value) {
    //     createDataFolder();
    //     let data;
    //     try {
    //         data = JSON.parse(await readDataFile().catch(() => "{}"));
    //     } catch(e) {} finally {
    //         if(!data) data = {};
    //         data[key] = value;
    //         setFileData({...data});
    //     }
    // }

    // const createDataFolder = async () => {
    //     await createDir("CodeMacros", {
    //         dir: BaseDirectory.Config,
    //         recursive: true,
    //     })
    // };

    // const setFileData = async (data) => {
    //     return writeFile(
    //         {
    //             contents: JSON.stringify(data),
    //             path: `CodeMacros/config.json`,
    //         },
    //         {
    //             dir: BaseDirectory.Config,
    //         }
    //     );
    // };

    const readDataFile = () => {
        return readTextFile("CodeMacros/Logs/log.txt", {
            // TODO: Make this BaseDirectory.Log instead of BaseDirectory.Config because that's what Rust writes to. 
            // For now, this works on Windows and Linux which are the target OSes.
            dir: BaseDirectory.Config
        })
    }

    let logText = ref(null);

    getLog().then((data) => {
        logText.value.innerHTML = data.replace(/\n/g, "<br>");
    });
</script>

<template>
    <div class="logText" ref="logText"></div>

    <span class="backButton" @click="goBack">&lt;</span>
</template>

<style scoped>
    .backButton {
        position: absolute;
        top: -10px;
        left: 10px;
        font-size: 50px;
        font-weight: 800;
        color: var(--secondary-text);
        cursor: pointer;
    }
    .backButton:hover {
        color: var(--primary-text);
    }
    .logText {
        padding: 45px 15px 15px 15px;
        font-size: 16px;
        font-family: monospace;
        color: var(--secondary-text);
    }
</style>