<script setup>
  import { reactive, onUnmounted } from 'vue';

  import CodeArea from './CodeArea.vue';

  import codeTypes from '../data/codeTypes.json';

  const props = defineProps(["executes", "title", "openArgumentsPopup"]);

  const executes = reactive(props.executes);

  function addCode(codeType) {
    let codeInside = {};
    for(let i = 0; i < codeType?.codeInside?.length; i++) {
      codeInside[codeType.codeInside[i].value] = {
        executes: []
      };
    }
    let variables = [];
    for(let i = 0; i < codeType?.variables?.length; i++) {
      variables.push({
        type: codeType.variables[i].value,
        name: codeType.variables[i].name
      })
    }
    let data = {};
    let defaultTypeValues = {
      string: 'String',
      number: 0, 
      condition: { type: 'boolean', value: true }, 
      function: 'Function'
    }
    for(let i = 0; i < codeType.parameters.length; i++) {
      let value = codeType.parameters[i].defaultValue;
      if(value === undefined) {
        value = defaultTypeValues[codeType.parameters[i].type];
      }
      data[codeType.parameters[i].value] = value;
    }
    executes.push({
      type: codeType.value,
      data: data,
      variables: variables,
      codeInside: codeInside
    });
  }

  function deleteCode(index) {
    executes.splice(index, 1);
  }

  let currentlyDragging = null;
  function startDrag(e, index) {
    let target = e.target;
    while(target !== null) {
      if(target.classList.contains('code')) {
        currentlyDragging = {
          elem: target,
          top: target.getBoundingClientRect().top,
          index: index,
          tempExecutes: JSON.parse(JSON.stringify(executes))
        }
        return;
      }
      target = target.parentElement;
    }

    if(currentlyDragging === null) return

    currentlyDragging.elem.style.opacity = 0.5;
  }

  window.addEventListener('mouseup', stopDrag);
  function stopDrag() {
    if(currentlyDragging === null) return;
    currentlyDragging.elem.style.opacity = 1;
    currentlyDragging.elem.style.top = "";
    for(let i = 0; i < currentlyDragging.tempExecutes.length; i++) {
      executes[i] = currentlyDragging.tempExecutes[i];
    }
    currentlyDragging = null;
  }

  window.addEventListener('mousemove', drag);
  function drag(e) {
    if(currentlyDragging) {
      let siblings = currentlyDragging.elem.parentElement.children;
      for(let i = 0; i < siblings.length; i++) {
        if(siblings[i] === currentlyDragging.elem) continue;
        if(siblings[i].getBoundingClientRect().top > e.clientY) {
          siblings[i].before(currentlyDragging.elem);
          currentlyDragging.top = siblings[i].getBoundingClientRect().top - currentlyDragging.elem.getBoundingClientRect().height;
          let element = currentlyDragging.tempExecutes[currentlyDragging.index];
          let index = i - 1 >= 0 ? i - 1 : 0;
          currentlyDragging.tempExecutes.splice(currentlyDragging.index, 1);
          currentlyDragging.tempExecutes.splice(index, 0, element);
          currentlyDragging.index = index;
          break;
        }
        if(i === siblings.length - 1) {
          siblings[i].after(currentlyDragging.elem);
          currentlyDragging.top = siblings[i].getBoundingClientRect().top - siblings[i].getBoundingClientRect().height;
          let element = currentlyDragging.tempExecutes[currentlyDragging.index];
          currentlyDragging.tempExecutes.splice(currentlyDragging.index, 1);
          currentlyDragging.tempExecutes.splice(i, 0, element);
          currentlyDragging.index = i;
        }
      }
      currentlyDragging.elem.style.top = e.clientY - currentlyDragging.top + 'px';
    }
  }
  
  onUnmounted(() => {
    window.removeEventListener('mouseup', stopDrag);
    window.removeEventListener('mousemove', drag);
  });

  const getCodeType = (execute) => codeTypes.find(codeType => codeType.value === execute?.type);
</script>

<template>
  <div class="codeArea">
    <h2 v-if="props.title" class="codeAreaTitle">{{props.title}}</h2>
    <div>
      <span v-if="executes.length === 0">This doesn't execute anything. Hover over add code to add something for it to execute.</span>
      <div v-else>
        <div 
          v-for="(execute, index) in executes" 
          :key="execute" 
          class="code">
            <svg
              @mousedown="(e) => startDrag(e, index)"
              class="dragCode"
              xmlns="http://www.w3.org/2000/svg" aria-hidden="true" role="img" style="vertical-align: -0.125em;" 
              width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
                <path fill="currentColor" d="M21 11H3V9h18v2m0 2H3v2h18v-2Z"/>
            </svg>
            <div class="codeType" :class="execute.type">
              <span>{{getCodeType(execute)?.name || "Unknown"}}</span>
            </div>
            <div
              v-for="(executesIteration, key) in execute.codeInside"
              :key="key">
              <CodeArea :openArgumentsPopup="props.openArgumentsPopup" :title="getCodeType(execute)?.codeInside?.find(codeInside => codeInside.value === key)?.name" :executes="executesIteration.executes" />
            </div>
            <svg 
              class="settings"
              @click="props.openArgumentsPopup(execute)"
              xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
              <g fill="none" stroke="currentColor" stroke-linecap="round" stroke-width="2">
                <path d="M3 5h4m14 0H11m-8 7h12m6 0h-2M3 19h2m16 0H9"/>
                <circle cx="9" cy="5" r="2"/>
                <circle cx="17" cy="12" r="2"/>
                <circle cx="7" cy="19" r="2"/>
              </g>
            </svg>
            <svg 
              class="deleteCode"
              @click="deleteCode(index)"
              xmlns="http://www.w3.org/2000/svg" aria-hidden="true" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
                <path fill="#9b3434" d="M19 4h-3.5l-1-1h-5l-1 1H5v2h14M6 19a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7H6v12Z"/>
            </svg>
        </div>
      </div>
    </div>
    <span class="addCode">
      <span>Add code</span>
      <div class="addCodeSelect">
        <div 
          v-for="codeType in codeTypes" 
          class="addCodeSelectOption"
          :key="codeType" 
          @click="addCode(codeType)">
            <span>{{ codeType.name }}</span>
            <p>{{ codeType.description }}</p>
        </div>
      </div>
    </span>
  </div>
</template>

<style scoped>
  .code {
    background-color: var(--dark-background);
    margin: 5px;
    padding: 5px;
    position: relative;
    font-size: 18px;
    margin-right: 10px;
  }
  .deleteCode {
    position: absolute;
    right: 5px;
    top: 5px;
  }
  .settings {
    position: absolute;
    right: 25px;
    top: 5px;
  }
  .dragCode {
    margin-right: 5px;
    color: gray;
    cursor: grab;
    width: 20px;
    height: 20px;
  }
  .codeType {
    display: inline-block;
  }
  .deleteCode:hover path {
    cursor: pointer;
    fill: #b62d2d;
  }
  .codeArea {
    width: 100%;
    padding: 5px 0 5px 5px;
    margin-top: 5px;
    background-color: var(--primary-background);
  }
  .addCode {
    border: 3px solid #141a2766;
    margin-top: 10px;
    padding: 5px;
    padding: 0 25px 0 10px;
    position: relative;
    width: 100%;
    height: 35px;
    display: inline-block;
    font-size: 22px;
    text-align: center;
  }
  .addCode::after {
    position: absolute;
    content: "";
    /* Down arrow */
    width: 0;
    height: 0;
    --size: 8px;
    border-left: var(--size) solid transparent;
    border-right: var(--size) solid transparent;
    border-top: var(--size) solid #fff;
    top: calc(50% - var(--size) / 2);
    right: 15px;
  }
  .addCodeSelect {
    position: absolute;
    background-color: #222a3a99;
    top: 28px;
    left: -3px;
    width: calc(100% + 6px);
    display: none;
    flex-wrap: wrap;
    z-index: 200;
  }
  .addCodeSelectOption {
    margin: 5px;
    padding: 5px;
    width: 296px;
    background-color: #191b1f99;
    cursor: pointer;
  }
  .addCodeSelectOption:hover {
    background-color: #191b1fcc;
  }
  .addCodeSelectOption span {
    font-size: 30px;
    font-weight: bold;
    color: #fff;
    margin: 0;
  }
  .addCodeSelectOption p {
    font-size: 20px;
    color: #ddd;
    margin: 10px;
  }
  .addCode:hover .addCodeSelect {
    display: flex;
  }
  .codeAreaTitle {
    margin: 0;
    font-size: 25px;
    font-weight: bold;
  }
</style>