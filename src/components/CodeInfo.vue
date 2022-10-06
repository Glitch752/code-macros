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
                <span v-if=     "infoItem.type === 'variable'" class="variable chip">{{infoItem.variable}}</span>
                <span v-else-if="infoItem.type === 'string'"   class="string   chip">{{infoItem.string}}  </span>
                <span v-else>{{infoItem}}</span>
            </span>
        </span>
    </template>
    <span v-else>{{info}}</span>
</template>

<style scoped>
    .chip {
        border-radius: 100vw;
        padding: 0 5px;
    }
    .chip.variable {
        background-color: #28569c;
    }
    .chip.string {
        background-color: #575656;
    }
    .section {
        margin-right: 5px;
    }
</style>