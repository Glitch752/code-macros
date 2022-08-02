<script setup>
  import { ref, watch } from 'vue';

  const props = defineProps(["selectedMacro", "setMacro"]);

  const selectedMacro = ref(props.selectedMacro);

  watch(selectedMacro, (newValue) => {
    props.setMacro(newValue);
  }, { deep: true });
</script>

<template>
  <div v-if="selectedMacro === null">
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
