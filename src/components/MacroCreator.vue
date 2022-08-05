<script setup>
  import { ref, watch } from 'vue';
  import MacroInitiator from '@/components/MacroInitiator.vue';
  import MacroFunction from '@/components/MacroFunction.vue';

  const props = defineProps(["selectedMacro", "setMacro"]);

  const selectedMacro = ref(props.selectedMacro);

  watch(selectedMacro, (newValue) => {
    props.setMacro(newValue);
  }, { deep: true });

  function getData(macro) {
    return {
      initiators: macro?.macro?.initiators || [],
      functions: macro?.macro?.functions || []
    };
  }

  function addInitiator() {
    let initiators = getData(selectedMacro.value).initiators;
    initiators.push({
      type: "nothing",
      data: {},
      executes: []
    });
    if(!selectedMacro.value.macro) selectedMacro.value.macro = {};
    selectedMacro.value.macro.initiators = initiators;
  }

  function addFunction() {
    let functions = getData(selectedMacro.value).functions;
    functions.push({
      name: "Function",
      parameters: [],
      executes: []
    });
    if(!selectedMacro.value.macro) selectedMacro.value.macro = {};
    selectedMacro.value.macro.functions = functions;
  }

  function deleteInitiator(initiator) {
    let initiators = getData(selectedMacro.value).initiators;
    initiators.splice(initiators.indexOf(initiator), 1);
    if(!selectedMacro.value.macro) selectedMacro.value.macro = {};
    selectedMacro.value.macro.initiators = initiators;
  }

  function deleteFunction(function_) {
    let functions = getData(selectedMacro.value).functions;
    functions.splice(functions.indexOf(function_), 1);
    if(!selectedMacro.value.macro) selectedMacro.value.macro = {};
    selectedMacro.value.macro.functions = functions;
  }
</script>

<template>
  <div class="background" v-if="selectedMacro === null">
    Select a macro to get started!
  </div>
  <div class="background" v-else>
    <input class="macroName" type="text" v-model="selectedMacro.name" placeholder="Name"/>
    <input class="macroDescription" type="text" v-model="selectedMacro.description" placeholder="Description"/>
    <h2>Initiators</h2>
    <div v-for="initiator in getData(selectedMacro).initiators" :key="initiator">
      <MacroInitiator :initiator="initiator" :deleteInitiator="deleteInitiator"/>
    </div>
    <button class="addInitiator" @click="addInitiator">Add initiator</button>
    <h2>Functions</h2>
    <div v-for="function_ in getData(selectedMacro).functions" :key="function_">
      <MacroFunction :function_="function_" :deleteFunction="deleteFunction"/>
    </div>
    <button class="addFunction" @click="addFunction">Add function</button>
  </div>
</template>

<style scoped>
  .background {
    width: 100%;
    height: 100%;
    background-color: #24293577;
  }
  h2 {
    margin: 0;
    margin-top: 20px;
    padding: 0;
    text-align: center;
    color: #fff;
    font-size: 24px;
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
  .addInitiator, .addFunction {
    background-color: transparent;
    color: #fff;
    border: none;
    outline: 3px solid #292f3d;
    width: calc(100% - 20px);
    font-size: 1.5rem;
    font-weight: 500;
    margin: 10px;
    cursor: pointer;
  }
  .addInitiator:hover, .addFunction:hover {
    outline: 3px solid #32394b;
  }
</style>
