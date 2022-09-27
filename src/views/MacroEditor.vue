<script setup>
  import * as store from '@/store';
  import { ref, onUnmounted } from 'vue';
  import MacroCreator from '@/components/MacroCreator.vue';
  import DraggingCode from '@/components/DraggingCode.vue'
  import updateMacros from '../utils';

  import CodeArgumentsPopup from '@/components/CodeArgumentsPopup.vue';
  import CodeList from '@/components/CodeList.vue'

  import { useRoute, useRouter } from 'vue-router';

  let showPopup = ref(false);

  let macrosLoaded = ref(false);
  let macros = ref([]);
  let selectedMacro = ref(null);

  let draggingCodeContainer = ref(null);

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

  function openArgumentsPopup(execute) {
    showPopup.value = execute;
  }
  
  function closePopup() {
    showPopup.value = false;
  }

  let draggingCode = ref(false);

  function dragCode(codeType) {
    draggingCode.value = codeType;
  }

  document.addEventListener("mousemove", mouseMove);
  document.addEventListener("mouseup",   mouseUp  );

  onUnmounted(() => {
    document.removeEventListener("mousemove", mouseMove);
    document.removeEventListener("mouseup",   mouseUp  );
  });

  function mouseMove(e) {
    if(draggingCodeContainer.value) {
      draggingCodeContainer.value.style.top  = e.clientY + "px";
      draggingCodeContainer.value.style.left = e.clientX + "px";
    }
  }

  function mouseUp() {
    console.log("Ran");
    draggingCode.value = false;
  }
</script>

<template>
  <div class="leftPane">
    <!-- TODO: Make this a list of code -->
    <h1>Code</h1>
    <CodeList :dragCode="dragCode" />
  </div>
  <div class="rightPane">
    <MacroCreator :openArgumentsPopup="openArgumentsPopup" v-if="macrosLoaded" :selectedMacro="selectedMacro" :setMacro="setMacro" :deleteMacro="() => deleteMacro(selectedMacroIndex)" :key="selectedMacroIndex"/>
  </div>
  <span class="backButton" @click="goBack">&lt;</span>
  <CodeArgumentsPopup v-if="showPopup !== false" :execute="showPopup" :close="closePopup"/>
  <div ref="draggingCodeContainer" class="draggingCodeContainer">
    <DraggingCode v-if="draggingCode" :code="draggingCode" />
  </div>
</template>

<style scoped>
  .draggingCodeContainer {
    position: absolute;
    z-index: 2000;

    transform: translate(-50%, -50%)
  }
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