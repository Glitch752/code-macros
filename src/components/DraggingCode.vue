<script setup>
    import { ref } from "vue";
    import codeTypes from '../data/codeTypes';

    const props = defineProps(["code"]);

    const code = ref(props.code);
    
    const getCodeType = (execute) => codeTypes.find(codeType => codeType.value === execute);

    const name = getCodeType(code.value)?.name;

    console.log(getCodeType(code.value));
</script>

<template>
    <span class="name">{{name}}</span>
</template>

<style scoped>

</style>