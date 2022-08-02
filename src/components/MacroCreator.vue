<script setup>
  import { reactive, watch } from 'vue';
  import * as store from '@/store';

  const props = defineProps(["selectedMacro", "macros", "macroIndex"]);

  const selectedMacro = reactive(props.selectedMacro || {});

  watch(selectedMacro, (newValue, oldValue) => {
    console.log(selectedMacro, props.macros);
    let newMacro = {...selectedMacro};
    let newMacros = [...props.macros];
    newMacros[props.macroIndex] = newMacro;
    store.set('macros', newMacros);
  });
</script>

<template>
  <div v-if="selectedMacro === {}">
    Select a macro to get started!
  </div>
  <div v-else>
    <h1>{{ selectedMacro.name }}</h1>
    <input type="text" v-model="selectedMacro.name" placeholder="Name"/>
  </div>
</template>

<style scoped>
  div {
    width: 100%;
    height: 100%;
    background-color: #24293577;
  }
</style>
