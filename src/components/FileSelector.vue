<script setup>
    import { toRefs } from 'vue';

    import { open } from '@tauri-apps/api/dialog';

    const props = defineProps(["filterName", "filterExtensions"]);
    const { filterName, filterExtensions } = toRefs(props);

    const emit = defineEmits(["selected"]);

    async function selectFile() {
        const selected = await open({
            multiple: false,
            filters: [{
                name: filterName.value,
                extensions: filterExtensions.value
            }]
        });
        
        if (selected === null) {
            return;
        }

        emit("selected", {
            path: selected
        });
    }
</script>

<template>
    <button @click="selectFile">Select file</button>
</template>

<style scoped>

</style>