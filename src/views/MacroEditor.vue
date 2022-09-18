<script setup>
  import * as store from '@/store';
  import { ref } from 'vue';
  import MacroCreator from '@/components/MacroCreator.vue';
  import updateMacros from '../utils';

  import { useRoute, useRouter } from 'vue-router';

  let macrosLoaded = ref(false);
  let macros = ref([]);
  let selectedMacro = ref(null);

  const route = useRoute();
  let router = useRouter();

  let selectedMacroIndex = route.params.macroIndex;
  
  store.get('macros', []).then(data => {
    macros.value = data;
    macrosLoaded.value = true;

    selectedMacro.value = macros.value[selectedMacroIndex];
    console.log(selectedMacro.value);
  });

  function setMacro(macro) {
    throttle(() => {
      const currentMacros = [...macros.value];
      currentMacros[macro.index] = {...macro, index: undefined};
      store.set('macros', currentMacros).then(() => {
        updateMacros();
      });
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

  function deleteMacro(index) {
    const currentMacros = [...macros.value];
    currentMacros.splice(index, 1);
    store.set('macros', currentMacros).then(() => {
      updateMacros();
    });
    macros.value = currentMacros;

    router.push("/macros");
  }

  function goBack() {
    router.push("/macros");
  }
</script>

<template>
  <div class="leftPane">
    <!-- TODO: Make this a list of code -->
    <h1>Code</h1>
  </div>
  <div class="rightPane">
    <MacroCreator v-if="macrosLoaded" :selectedMacro="selectedMacro" :setMacro="setMacro" :deleteMacro="() => deleteMacro(selectedMacroIndex)" :key="selectedMacroIndex"/>
  </div>
  <span class="backButton" @click="goBack">&lt;</span>
</template>

<style scoped>
  .leftPane {
    position: relative;
    display: inline-block;
    width: 300px;
    height: calc(100% - 24px);
  }
  .rightPane {
    position: absolute;
    display: inline-block;
    width: calc(100% - 300px);
    height: calc(100% - 24px);
  }
  h1 {
    text-align: center;
    margin: 0;
  }
  .backButton {
    position: absolute;
    top: -10px;
    left: 10px;
    font-size: 50px;
    font-weight: 800;
    color: #ccc;
    cursor: pointer;
  }
  .backButton:hover {
    color: #fff;
  }
</style>