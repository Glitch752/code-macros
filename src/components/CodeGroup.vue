<script setup>
    import codeTypes from '../data/codeTypes';
    import { toRefs, ref } from 'vue';

    let props = defineProps(["codeGroupData"]);
    let { codeGroupData } = toRefs(props);

    function codeTypeValue(value) {
        return codeTypes.find(codeType => codeType.value == value);
    }

    const expanded = ref(false);
</script>

<template>
    <div class="codeGroup">
        <span class="groupName">{{codeGroupData.name}}</span>
        <!-- Expand -->
        <svg @click="expanded = true" v-if="!expanded" class="groupToggle expand" xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 1024 1024">
            <path fill="currentColor" d="M715.8 493.5L335 165.1c-14.2-12.2-35-1.2-35 18.5v656.8c0 19.7 20.8 30.7 35 18.5l380.8-328.4c10.9-9.4 10.9-27.6 0-37z"/>
        </svg>
        <!-- Retract -->
        <svg @click="expanded = false" v-if="expanded" class="groupToggle retract" xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 1024 1024">
            <path fill="currentColor" d="M840.4 300H183.6c-19.7 0-30.7 20.8-18.5 35l328.4 380.8c9.4 10.9 27.5 10.9 37 0L858.9 335c12.2-14.2 1.2-35-18.5-35z"/>
        </svg>
        <template v-if="expanded">
            <div class="codeType" v-for="codeType in codeGroupData.code" :key="codeType.value">
                <span class="name">{{codeTypeValue(codeType).name}}</span>
                <span class="description">{{codeTypeValue(codeType).description}}</span>
            </div>
        </template>
    </div>
</template>

<style scoped>
    .codeGroup {
        width: calc(100% - 10px);
        background-color: var(--dark-background);
        margin: 5px;
        padding: 5px;
        position: relative;
    }
    .groupName {
        color: var(--primary-text);
        font-size: 20px;
        display: block;
        margin-left: 5px;
    }
    .groupToggle {
        position: absolute;
        top: 5px;
        right: 5px;
        width: 20px;
        height: 20px;
    }
    .codeType {
        width: calc(100% - 10px);
        background-color: var(--primary-background);
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