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
    if(draggingCode.value !== false) {
      if(draggingCodeContainer.value) {
        draggingCodeContainer.value.style.top  = e.clientY + "px";
        draggingCodeContainer.value.style.left = e.clientX + "px";
      }
      
      let codeAreas = document.querySelectorAll("[data-code-area]");

      let codeAreasSorted = Array.from(codeAreas).sort((a, b) => b.dataset.codeArea - a.dataset.codeArea);

      // Find the topmost codearea the mouse is hovering over
      for(let i = 0; i < codeAreasSorted.length; i++) {
        let codeArea = codeAreasSorted[i];

        let boundingBox = codeArea.getBoundingClientRect();

        if(boundingBox.left < e.clientX && boundingBox.top < e.clientY && boundingBox.left + boundingBox.width > e.clientX && boundingBox.top + boundingBox.height > e.clientY) {
          // Loop over the elements and find the gap between elements we are closest to

          // FIXME: Is there a more VueJS-based solution to this?
          if(codeArea.dataset.noCode === "true") {
              let placeholder = createPlaceholder(0);
              if(placeholder) {
                codeArea.querySelector("[data-code]").appendChild(placeholder);
              }
          } else {
            let children = codeArea.querySelector("[data-code]").children;
            for(let j = 0; j < children.length; j++) {
              let child = children[j];

              // If the element is the placeholder, continue to the next element.
              if(child.dataset.insertPlaceholder) continue

              let childBoundingBox = child.getBoundingClientRect();

              if(childBoundingBox.top > e.clientY) {
                // Create a placeholder element
                let placeholder = createPlaceholder(j);

                if(!placeholder) break;

                child.before(placeholder)

                break;
              } else if(j === children.length - 1) {
                // Create a placeholder element
                let placeholder = createPlaceholder(j + 1);
                if(!placeholder) break;

                child.after(placeholder)

                break;
              }
            }
          }

          break;
        }
      }
    }
  }

  function createPlaceholder(j) {
    let currentPlaceholder = document.querySelector("[data-insert-placeholder]");

    let placeholderPosition = j;

    if(currentPlaceholder) {
      if(placeholderPosition == currentPlaceholder.dataset.placeholderPosition) {
        return;
      } else {
        currentPlaceholder.remove();
      }
    }

    return elementFromHTMLString(`<div class="insertPlaceholder" data-insert-placeholder data-placeholder-position="${placeholderPosition}"></div>`)
  }

  function elementFromHTMLString(string) {
    var temp = document.createElement('template');
    string = string.trim();
    temp.innerHTML = string;
    return temp.content.firstChild;
  }

  function mouseUp() {
    console.log("Ran");
    draggingCode.value = false;

    let currentPlaceholder = document.querySelector("[data-insert-placeholder]");
    if(currentPlaceholder) {
      currentPlaceholder.remove();
    }
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