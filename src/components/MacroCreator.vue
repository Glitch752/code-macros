<script setup>
  import { ref, watch } from 'vue';
  import MacroInitiator from '@/components/MacroInitiator.vue';
  import MacroFunction from '@/components/MacroFunction.vue';

  const props = defineProps(["selectedMacro", "setMacro", "deleteMacro", "openArgumentsPopup"]);

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
    <svg 
      class="deleteMacro"
      @click="props.deleteMacro()"
      xmlns="http://www.w3.org/2000/svg" aria-hidden="true" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
        <path fill="#9b3434" d="M19 4h-3.5l-1-1h-5l-1 1H5v2h14M6 19a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7H6v12Z"/>
    </svg>
    <input class="macroName" type="text" v-model="selectedMacro.name" placeholder="Name"/>
    <input class="macroDescription" type="text" v-model="selectedMacro.description" placeholder="Description"/>
    <h2>Initiators</h2>
    <div v-for="(initiator, index) in getData(selectedMacro).initiators" :key="initiator">
      <MacroInitiator :index="index" :openArgumentsPopup="props.openArgumentsPopup" :initiator="initiator" :deleteInitiator="deleteInitiator"/>
    </div>
    <button class="addInitiator" @click="addInitiator">Add initiator</button>
    <h2>Functions</h2>
    <div v-for="(function_, index) in getData(selectedMacro).functions" :key="function_">
      <MacroFunction :index="index" :openArgumentsPopup="props.openArgumentsPopup" :function_="function_" :deleteFunction="deleteFunction"/>
    </div>
    <button class="addFunction" @click="addFunction">Add function</button>
  </div>
</template>

<style scoped>
  .deleteMacro {
    position: absolute;
    top: 5px;
    right: 5px;
    width: 40px;
    height: 40px;
  }
  .deleteMacro:hover path {
    cursor: pointer;
    fill: #b62d2d;
  }
  .background {
    width: 100%;
    height: 100%;
    background-color: var(--secondary-background);
    overflow-y: auto;
  }
  h2 {
    margin: 0;
    margin-top: 20px;
    padding: 0;
    text-align: center;
    color: var(--primary-text);
    font-size: 24px;
  }
  .macroName {
    border: none;
    outline: 2px solid transparent;
    background: transparent;
    color: var(--primary-text);
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
    outline: 2px solid var(--accent-light);
  }
  .addInitiator, .addFunction {
    background-color: transparent;
    color: var(--primary-text);
    border: none;
    outline: 3px solid var(--accent);
    width: calc(100% - 20px);
    font-size: 1.5rem;
    font-weight: 500;
    margin: 10px;
    cursor: pointer;
  }
  .addInitiator:hover, .addFunction:hover {
    outline: 3px solid var(--accent-light);
  }
</style>
