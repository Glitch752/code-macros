<script setup>
    import { ref } from "vue";

    let props = defineProps(["info"]);
    let info = ref(props.info);
</script>

<template>
    <template v-if="(typeof info !== 'string')">
        <span class="section" v-for="infoItem in info" :key="infoItem">
            <span v-if="typeof infoItem === 'string'">{{ infoItem }}</span>
            <span v-else>
                <span v-if="infoItem.type === 'variable'" class="variableChip">{{infoItem.variable}}</span>
                <span v-else>{{infoItem}}</span>
            </span>
        </span>
    </template>
    <span v-else>{{info}}</span>
</template>

<style scoped>
    .variableChip {
        background-color: #28569c;
        border-radius: 100vw;
        padding: 0 5px;
    }
    .section {
        margin-right: 5px;
    }
</style>