<script setup>
  import * as store from '@/store';
  import { ref } from 'vue';
  import MacroCreator from '@/components/MacroCreator.vue';
  import updateMacros from '../utils';

  let macrosLoaded = ref(false);
  let macros = ref([]);
  let selectedMacro = ref(null);
  let selectedMacroIndex = ref(null);
  store.get('macros', []).then(data => {
    macros.value = data;
    macrosLoaded.value = true;
  });

  function addMacro() {
    const currentMacros = macros.value;
    const newMacro = {
      name: 'New Macro',
      description: 'New description',
      macro: {
        initiators: [],
        functions: []
      }
    };
    currentMacros.push(newMacro);
    store.set('macros', currentMacros);
  }

  function selectMacro(macro) {
    let index = macros.value.indexOf(macro);
    macro.index = index;
    selectedMacro.value = macro;
    selectedMacroIndex.value = index;
  }

  function setMacro(macro) {
    throttle(() => {
      const currentMacros = [...macros.value];
      currentMacros[macro.index] = {...macro, index: undefined};
      store.set('macros', currentMacros);

      updateMacros();
    }, 1000);
  }

  let timeout = null;
  function throttle(func, wait) {
    if(timeout) clearTimeout(timeout);
    timeout = setTimeout(() => {
      func();
      timeout = null;
    }, wait);
  }
</script>

<template>
  <div class="leftPane">
    <h1>Macros</h1>
    <div class="noMacros" v-if="macrosLoaded && macros.length === 0">
      <p>No macros found. Add a new one?</p>
      <button class="addMacro button" @click="addMacro">Add macro</button>
    </div>
    <div v-else-if="macrosLoaded">
      <div v-for="macro in macros" class="macro" :class="{ activeMacro: macro === selectedMacro }" :key="macro" @click="selectMacro(macro)">
        <div class="macroName">{{ macro.name }}</div>
        <div class="macroDescription">{{ macro.description }}</div>
      </div>
      <div class="addMacroButton macro" @click="addMacro">
        <span>+++++++++</span> Add macro
      </div>
    </div>
    <div class="loading" v-else>
      <div class="loadingText">
        Loading macros...
      </div>
    </div>
  </div>
  <div class="rightPane">
    <MacroCreator v-if="macrosLoaded" :selectedMacro="selectedMacro" :setMacro="setMacro" :key="selectedMacroIndex"/>
  </div>
</template>

<style scoped>
  .macro {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 10px;
    border: 3px solid #242935;
    margin: 10px;
    cursor: pointer;
  }
  .macro:hover {
    border: 3px solid #292f3d;
  }
  .activeMacro {
    border: 3px solid #40495f;
  }
  .activeMacro:hover {
    border: 3px solid #40495f;
  }
  .macroDescription {
    font-size: 12px;
    color: #8e959c;
  }
  h1 {
    text-align: center;
    margin: 0;
  }
  .leftPane {
    position: relative;
    display: inline-block;
    width: 300px;
    height: 100%;
  }
  .rightPane {
    position: absolute;
    display: inline-block;
    width: calc(100% - 300px);
    height: 100%;
  }
  .noMacros {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding-bottom: 100px;
  }
  .addMacro {
    --defaultBG: #2c9094;
    --defaultBoxBG: #237477;
    --hoverBG: #2fa3a3;
    --hoverBoxBG: #2a8a8d;
    --activeBG: #23787e;
    --activeBoxBG: #1c5f5f;
  }
  .addMacroButton {
    height: 50px;
    font-size: 30px;
    font-weight: 500;
    color: #bbb;
  }
  .addMacroButton:hover {
    color: #fff;
  }
  .addMacroButton span {
    font-size: 60px;
    color: #a7aebd77;
    position: absolute;
    z-index: -1;
    font-weight: bold;
  }
</style>