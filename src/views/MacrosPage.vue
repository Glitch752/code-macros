<script setup>
  import * as store from '@/store';
  import { ref } from 'vue';
  import MacroCreator from '@/components/MacroCreator.vue';

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
    selectedMacro.value = macro;
    selectedMacroIndex.value = macros.value.indexOf(macro);
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
        <div class="macroName">
          {{ macro.name }}
        </div>
        <div class="macroDescription">
          {{ macro.description }}
        </div>
      </div>
    </div>
    <div class="loading" v-else>
      <div class="loadingText">
        Loading macros...
      </div>
    </div>
  </div>
  <div class="rightPane">
    <MacroCreator v-if="macrosLoaded" :selectedMacro="selectedMacro" :macros="macros" :macroIndex="selectedMacroIndex"/>
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
</style>