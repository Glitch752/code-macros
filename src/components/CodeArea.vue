<script setup>
  import { toRefs, onUnmounted } from 'vue';

  const props = defineProps(["executes"]);

  const { executes } = toRefs(props);

  const codeTypes = [
    {name: 'If', value: 'if', description: 'Executes the code inside if the condition is true.'},
    {name: 'Function call', value: 'function', description: 'Executes the code inside the function it calls.'},
    {name: 'Repeat N times loop', value: 'ntimesloop', description: 'Executes the code inside the loop a certain number of times.'},
    {name: 'Repeat while loop', value: 'whileloop', description: 'Executes the code inside the loop while the condition is true.'},
    {name: 'Notification', value: 'notification', description: 'Displays a notification.'},
    {name: 'Wait', value: 'wait', description: 'Waits for a certain amount of time.'},
    {name: 'Stop', value: 'stop', description: 'Stops the macro.'}
  ];

  function addCode(codeType) {
    executes.value.push({
      type: codeType.value,
      data: {}
    });
  }

  function deleteCode(index) {
    executes.value.splice(index, 1);
  }

  let currentlyDragging = null;
  function startDrag(e) {
    let target = e.target;
    while(target !== null) {
      if(target.classList.contains('code')) {
        currentlyDragging = {
          elem: target,
          top: target.getBoundingClientRect().top
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
          break;
        }
        if(i === siblings.length - 1) {
          siblings[i].after(currentlyDragging.elem);
          currentlyDragging.top = siblings[i].getBoundingClientRect().top - siblings[i].getBoundingClientRect().height;
        }
      }
      currentlyDragging.elem.style.top = e.clientY - currentlyDragging.top + 'px';
    }
  }
  
  onUnmounted(() => {
    window.removeEventListener('mouseup', stopDrag);
    window.removeEventListener('mousemove', drag);
  });
</script>

<template>
  <div class="codeArea">
    <div>
      <span v-if="executes.length === 0">This doesn't execute anything. Hover over add code to add something for it to execute.</span>
      <template v-else>
        <div 
          v-for="(execute, index) in executes" 
          :key="execute" 
          class="code">
            <svg
              @mousedown="startDrag"
              class="dragCode"
              xmlns="http://www.w3.org/2000/svg" aria-hidden="true" role="img" style="vertical-align: -0.125em;" 
              width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
                <path fill="currentColor" d="M21 11H3V9h18v2m0 2H3v2h18v-2Z"/>
            </svg>
            <div class="codeType" :class="execute.type">
              <span>{{codeTypes.find(codeType => codeType.value === execute.type).name}}</span>
            </div>
            <svg 
              class="deleteCode"
              @click="deleteCode(index)"
              xmlns="http://www.w3.org/2000/svg" aria-hidden="true" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
                <path fill="#9b3434" d="M19 4h-3.5l-1-1h-5l-1 1H5v2h14M6 19a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7H6v12Z"/>
            </svg>
        </div>
      </template>
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
    background-color: #171b2777;
    margin: 5px;
    padding: 5px;
    position: relative;
  }
  .deleteCode {
    position: absolute;
    right: 5px;
    top: 5px;
  }
  .dragCode {
    margin-right: 5px;
    color: gray;
    cursor: grab;
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
    padding: 5px;
    background-color: #1c1f2777;
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
</style>