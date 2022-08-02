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
    <input class="macroName" type="text" v-model="selectedMacro.name" placeholder="Name"/>
    <input class="macroDescription" type="text" v-model="selectedMacro.description" placeholder="Description"/>
  </div>
</template>

<style scoped>
  div {
    width: 100%;
    height: 100%;
    background-color: #24293577;
  }
  .macroName {
    border: none;
    outline: 2px solid transparent;
    background: transparent;
    color: #fff;
    font-size: 2rem;
    font-weight: 500;
    text-align: center;
    margin: 5px;
    width: calc(100% - 10px);
    transition: outline 0.3s cubic-bezier(.19,.9,.52,.91);
  }
  .macroDescription {
    border: none;
    outline: 2px solid transparent;
    background: transparent;
    color: #8e959c;
    font-size: 1rem;
    font-weight: 500;
    text-align: center;
    margin: 5px;
    width: calc(100% - 10px);
    transition: outline 0.3s cubic-bezier(.19,.9,.52,.91);
  }
  .macroName:hover, .macroDescription:hover, .macroName:focus, .macroDescription:focus {
    outline: 2px solid #223547;
  }
</style>
