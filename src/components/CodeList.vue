<script setup>
    import codeGroups from '../data/codeGroups.json';
    import CodeGroup from '@/components/CodeGroup.vue';

    const props = defineProps(["dragCode"]);
</script>

<template>
    <!-- TODO: This is a bit of an unnecessary component, so it could be combined in the future with something else -->
    <CodeGroup v-for="codeGroupData in codeGroups" :dragCode="props.dragCode" :codeGroupData="codeGroupData" :key="codeGroupData.value" />
</template>

<style scoped>
    
</style>