<script setup>
  import { reactive, onUnmounted } from 'vue';

  import CodeArea from './CodeArea.vue';
  import CodeInfo from './CodeInfo.vue';

  import codeTypes from '../data/codeTypes';

  const props = defineProps(["executes", "title", "openArgumentsPopup", "depth", "position"]);

  const executes = reactive(props.executes);

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

  const parsers = {
    parseCondition,
    parseExpression
  };

  function parseCondition(condition) {
    switch(condition.type) {
      case "comparison": {
        return [...parseExpression(condition.left), condition.comparison, ...parseExpression(condition.right)]
      }
      default: {
        return [JSON.stringify(condition)];
      }
    }
  }

  function parseExpression(expression) {
    switch(expression.type) {
      case "variable": {
        return [{ type: "variable", variable: expression.variable }];
      }
      case "number": {
        return [expression.value];
      }
      case "arithmetic": {
        const nameToSymbol = {
          "addition": "+",
          "subtraction": "-",
          "multiplication": "*",
          "division": "/",
          "modulo": "%",
          "exponent": "^"
        }
        return [...parseExpression(expression.left), nameToSymbol[expression.kind], ...parseExpression(expression.right)]
      }
      default: {
        return [JSON.stringify(expression)];
      }
    }
  }
</script>

<template>
  <div class="codeArea" :data-code-area="props.depth || 1" :data-no-code="executes.length === 0" :data-position="JSON.stringify(props.position)">
    <h2 v-if="props.title" class="codeAreaTitle">{{props.title}}</h2>
    <div>
      <span v-if="executes.length === 0" data-code>This doesn't execute anything yet.</span>
      <div v-else data-code>
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
            <span class="codeInfo">
              <CodeInfo :info="getCodeType(execute).contentText(execute.data, parsers)" :key="getCodeType(execute).contentText(execute.data, parsers)" />
            </span>
            <div
              v-for="(executesIteration, key) in execute.codeInside"
              :key="key">
              <CodeArea :depth="props.depth + 1 || 2" :position="{...props.position, treePosition: props.position.treePosition.concat(index).concat(key)}" :openArgumentsPopup="props.openArgumentsPopup" :title="getCodeType(execute)?.codeInside?.find(codeInside => codeInside.value === key)?.name" :executes="executesIteration.executes" />
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
  .codeAreaTitle {
    margin: 0;
    font-size: 25px;
    font-weight: bold;
  }

  .codeInfo {
    display: block;
    color: var(--secondary-text);
    margin-left: 25px;
    font-size: 15px;
  }
</style>