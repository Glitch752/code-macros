<script setup>
  import { ref, watch } from 'vue';

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
    let initiators = getData(selectedMacro).initiators;
    initiators.push({
      type: "nothing",
      data: {},
      executes: []
    });
    if(!selectedMacro.value.macro) selectedMacro.value.macro = {};
    selectedMacro.value.macro.initiators = initiators;
  }

  function addFunction() {
    let functions = getData(selectedMacro).functions;
    functions.push({
      name: "Function",
      parameters: [],
      executes: []
    });
    if(!selectedMacro.value.macro) selectedMacro.value.macro = {};
    selectedMacro.value.macro.functions = functions;
  }

  function deleteInitiator(initiator) {
    let initiators = getData(selectedMacro).initiators;
    initiators.splice(initiators.indexOf(initiator), 1);
    if(!selectedMacro.value.macro) selectedMacro.value.macro = {};
    selectedMacro.value.macro.initiators = initiators;
  }

  function deleteFunction(function_) {
    let functions = getData(selectedMacro).functions;
    functions.splice(functions.indexOf(function_), 1);
    if(!selectedMacro.value.macro) selectedMacro.value.macro = {};
    selectedMacro.value.macro.functions = functions;
  }

  const initiatorTypes = [
    {name: "Keypress", description: "Do something when a certain key combination is used.", value: "keypress"},
    {name: "Application Launched", description: "Do something when a certain application is launched.", value: "appLaunched"}
  ];
</script>

<template>
  <div class="background" v-if="selectedMacro === null">
    Select a macro to get started!
  </div>
  <div class="background" v-else>
    <input class="macroName" type="text" v-model="selectedMacro.name" placeholder="Name"/>
    <input class="macroDescription" type="text" v-model="selectedMacro.description" placeholder="Description"/>
    <h2>Initiators</h2>
    <div class="initiator" v-for="initiator in getData(selectedMacro).initiators" :key="initiator">
      <span class="initiatorType">
        <span>{{ initiatorTypes.find(initiatorType => initiatorType.value === initiator.type).name }}</span>
        <div class="initiatorSelect">
          <div 
            v-for="initiatorType in initiatorTypes" 
            class="initiatorSelectOption" 
            :class="{selected: initiator.type === initiatorType.value }"
            :key="initiatorType" 
            @click="initiator.type = initiatorType.value">
              <span>{{ initiatorType.name }}</span>
              <p>{{ initiatorType.description }}</p>
          </div>
        </div>
      </span>
      <svg 
        class="deleteInitiator" 
        @click="deleteInitiator(initiator)"
        xmlns="http://www.w3.org/2000/svg" aria-hidden="true" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
          <path fill="#9b3434" d="M19 4h-3.5l-1-1h-5l-1 1H5v2h14M6 19a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7H6v12Z"/>
      </svg>
    </div>
    <button class="addInitiator" @click="addInitiator">Add initiator</button>
    <h2>Functions</h2>
    <div class="function" v-for="function_ in getData(selectedMacro).functions" :key="function_">
      <input class="functionName" type="text" v-model="function_.name" placeholder="Name"/>
      <svg 
        class="deleteFunction"
        @click="deleteFunction(function_)"
        xmlns="http://www.w3.org/2000/svg" aria-hidden="true" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
          <path fill="#9b3434" d="M19 4h-3.5l-1-1h-5l-1 1H5v2h14M6 19a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7H6v12Z"/>
      </svg>
    </div>
    <button class="addFunction" @click="addFunction">Add function</button>
  </div>
</template>

<style scoped>
  .deleteInitiator, .deleteFunction {
    position: absolute;
    right: 8px;
    top: 10px;
    width: 30px;
    height: 30px;
  }
  .deleteInitiator:hover path, .deleteFunction:hover path {
    cursor: pointer;
    fill: #b62d2d;
  }
  .functionName {
    outline: 2px solid transparent;
    background: none;
    border: none;
    width: 500px;
    max-width: 40%;
    padding: 5px 10px;
    margin: 2px;
    font-size: 20px;
    color: #fff;
    transition: outline 0.3s cubic-bezier(.19,.9,.52,.91);
  }
  .functionName:hover, .functionName:focus {
    outline: 2px solid #223547;
  }
  .initiatorSelect {
    position: absolute;
    background-color: #222a3a99;
    top: 28px;
    left: -3px;
    width: 620px;
    display: none;
    flex-wrap: wrap;
  }
  .initiatorSelectOption {
    margin: 5px;
    padding: 5px;
    width: 300px;
    background-color: #191b1f99;
    cursor: pointer;
  }
  .initiatorSelectOption:hover {
    background-color: #191b1fcc;
  }
  .initiatorSelectOption.selected {
    background-color: #1f2229cc;
  }
  .initiatorSelectOption span {
    font-size: 40px;
    font-weight: bold;
    color: #fff;
    margin: 0;
  }
  .initiatorSelectOption p {
    font-size: 20px;
    color: #ddd;
    margin: 10px;
  }
  .initiatorType:hover .initiatorSelect {
    display: flex;
  }
  .initiator, .function {
    padding: 10px;
    margin: 10px;
    border: 3px solid #141a27;
    position: relative;
  }
  .initiator span, .function span {
    font-size: 20px;
  }
  .initiatorType {
    border: 3px solid #141a2766;
    margin: 5px;
    padding: 0 25px 0 10px;
    position: relative;
    width: 250px;
    display: inline-block;
  }
  .initiatorType::after {
    position: absolute;
    content: "";
    /* Down arrow */
    width: 0;
    height: 0;
    --size: 6px;
    border-left: var(--size) solid transparent;
    border-right: var(--size) solid transparent;
    border-top: var(--size) solid #fff;
    top: calc(50% - var(--size) / 2);
    right: 5px;
  }
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
