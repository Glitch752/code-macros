<script setup>
    import codeGroups from '../data/codeGroups.json';
    import CodeGroup from '@/components/CodeGroup.vue';

    const props = defineProps(["dragCode"]);
</script>

<template>
    <CodeGroup v-for="codeGroupData in codeGroups" :dragCode="props.dragCode" :codeGroupData="codeGroupData" :key="codeGroupData.value" />
</template>

<style scoped>
    
</style>