<script setup>
    import codeGroups from '../data/codeGroups.json';
    import CodeGroup from '@/components/CodeGroup.vue';

    const props = defineProps(["dragCode"]);
</script>

<template>
    <div class="scroll">
        <CodeGroup v-for="codeGroupData in codeGroups" :dragCode="props.dragCode" :codeGroupData="codeGroupData" :key="codeGroupData.value" />
    </div>
</template>

<style scoped>
    .scroll {
        overflow-y: scroll;
        height: calc(100% - 40px);
    }
</style>