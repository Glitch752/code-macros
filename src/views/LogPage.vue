<script setup>
    import { BaseDirectory, createDir, writeFile, readTextFile } from "@tauri-apps/api/fs";
    import { ref } from "vue";
    import { useRouter } from "vue-router";

    const router = useRouter();

    async function getLog() {
        return new Promise(async (resolve, reject) => {
            try {
                let data = await readDataFile();

                if (data) resolve(data);
            } catch(e) {
                resolve("No log data");
            }
        });
    }

    const readDataFile = () => {
        return readTextFile("CodeMacros/Logs/log.txt", {
            dir: BaseDirectory.Config
        })
    }

    const logText = ref(null);

    setlog();

    document.updateLog = () => {
        setlog();
    }

    function setlog() {
        getLog().then((data) => {
            logText.value.innerHTML = data.replace(/\n/g, "<br>");
        });
    }

    function goBack() {
        router.push("/macros");
    }
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