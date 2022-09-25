<script setup>
    import codeGroups from '../data/codeGroups.json';
    import CodeGroup from '@/components/CodeGroup.vue';
</script>

<template>
    <!-- TODO: This is a bit of an unnecessary component, so it could be combined in the future with something else -->
    <CodeGroup v-for="codeGroupData in codeGroups" :codeGroupData="codeGroupData" :key="codeGroupData.value" />
</template>

<style scoped>
    
</style>