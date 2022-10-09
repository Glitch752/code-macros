<script setup>
  import { toRefs } from 'vue';

  import CodeArea from './CodeArea.vue';

  const props = defineProps(["function_", "deleteFunction", "openArgumentsPopup", "index"]);

  const { function_, deleteFunction, index } = toRefs(props);
</script>

<template>
    <div class="function">
      <input class="functionName" type="text" v-model="function_.name" placeholder="Name"/>
      <svg 
        class="deleteFunction"
        @click="deleteFunction(function_)"
        xmlns="http://www.w3.org/2000/svg" aria-hidden="true" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
          <path fill="#9b3434" d="M19 4h-3.5l-1-1h-5l-1 1H5v2h14M6 19a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7H6v12Z"/>
      </svg>
      <br />
      <CodeArea :position="{ type: 'Function', treePosition: [index] }" :openArgumentsPopup="props.openArgumentsPopup" :executes="function_.executes" />
    </div>
</template>

<style scoped>

  .deleteFunction {
    position: absolute;
    right: 16px;
    top: 16px;
    width: 30px;
    height: 30px;
  }
  .deleteFunction:hover path {
    cursor: pointer;
    fill: #b62d2d;
  }
  .functionName {
    background-color: var(--primary-background);
    border: none;
    width: 500px;
    max-width: 40%;
    padding: 5px 10px;
    margin: 2px;
    font-size: 20px;
    color: var(--primary-text);
    transition: outline 0.3s cubic-bezier(.19,.9,.52,.91);
  }
  .functionName:hover, .functionName:focus {
    outline: 2px solid #223547;
  }
  .function {
    padding: 10px;
    margin: 10px;
    background-color: var(--dark-background);
    position: relative;
  }
  .function span {
    font-size: 20px;
  }
</style>