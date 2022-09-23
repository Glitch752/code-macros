<script setup>
    import codeTypes from '../data/codeTypes';
</script>

<template>
    <div class="codeType" v-for="codeType in codeTypes" :key="codeType.value">
        <span class="name">{{codeType.name}}</span>
        <span class="description">{{codeType.description}}</span>
    </div>
</template>

<style scoped>
    .codeType {
        width: calc(100% - 10px);
        background-color: var(--dark-background);
        margin: 5px;
        padding: 10px;
    }
    .name {
        color: var(--primary-text);
        font-size: 20px;
        display: block;
    }
    .description {
        color: var(--secondary-text);
        font-size: 16px;
    }
</style>