<script setup>
    import { reactive, toRefs } from 'vue';

    import codeTypes from '../data/codeTypes';

    import ConditionCreator from '@/components/ConditionCreator.vue';
    import ExpressionCreator from '@/components/ExpressionCreator.vue'
  
    const props = defineProps(["execute", "close"]);
  
    const execute = reactive(props.execute);
    
    const getCodeType = (execute) => codeTypes.find(codeType => codeType.value === execute?.type);
    const getParameter = (execute, argumentType) => getCodeType(execute)?.parameters?.find(parameter => parameter.value === argumentType);
    const getVariable = (execute, variableType) => getCodeType(execute)?.variables?.find(variable => variable.value === variableType);

    function checkClose(e) {
        let target = e.target;
        while(target.parentElement) {
            if(target.classList.contains("popup")) return
            target = target.parentElement;
        }

        // Close
        props.close();
    }
</script>

<template>
    <div class="popupContainer" @click="checkClose">
        <div class="popup">
            <div class="codeArguments">
                <div 
                    v-for="(argumentValue, argumentType, index) in toRefs(execute.data)" :key="index" 
                    :class="{codeArgument: ['condition', 'expression'].indexOf(getParameter(execute, argumentType)?.type) === -1}"
                >
                    <input 
                        v-if="getParameter(execute, argumentType)?.type === 'string'" 
                        v-model="argumentValue.value" 
                        type="text" 
                        class="codeArgumentInput"
                        :placeholder="getParameter(execute, argumentType)?.name" />
                    <input
                        v-if="getParameter(execute, argumentType)?.type === 'number'" 
                        v-model="argumentValue.value"
                        type="number"
                        class="codeArgumentInput"
                        :placeholder="getParameter(execute, argumentType)?.name" />
                    <!-- TODO: make this a dropdown of all possible functions -->
                    <input 
                        v-if="getParameter(execute, argumentType)?.type === 'function'" 
                        v-model="argumentValue.value" 
                        type="text" 
                        class="codeArgumentInput"
                        :placeholder="getParameter(execute, argumentType)?.name" />
                    <span
                        v-if="getParameter(execute, argumentType)?.type === 'condition'">
                        <ConditionCreator :condition="argumentValue" />  
                    </span>
                    <span
                        v-if="getParameter(execute, argumentType)?.type === 'expression'">
                        <ExpressionCreator :expression="argumentValue" />  
                    </span>
                    <select 
                        v-if="getParameter(execute, argumentType)?.type === 'multiSelect'"
                        :value="argumentValue.value" @change="(e) => argumentValue.value = e.target.value">
                            <option v-for="(option) in getParameter(execute, argumentType).options" :key="option.value" :value="option.value">{{ option.name }}</option>
                    </select>
                    <span
                        v-if="['condition', 'expression'].indexOf(getParameter(execute, argumentType)?.type) === -1">
                        {{getParameter(execute, argumentType)?.name || "Unknown"}}
                    </span>
                </div>
                <br />
                <span v-if="execute.variables.length > 0" style="display: block; margin-left: 5px;">Variables: </span>
                <div v-for="(variableValue, index) in toRefs(execute.variables)" :key="index" class="codeArgument">
                    <input 
                        v-model="variableValue.value.name" 
                        type="text" 
                        class="codeArgumentInput"
                        :placeholder="getVariable(execute, variableValue.value.type)?.name" />
                    <span 
                        v-if="['condition', 'expression'].indexOf(getParameter(execute, argumentType)?.type) === -1">
                        {{getVariable(execute, variableValue.value.type)?.name || "Unknown"}}
                    </span>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
    .popupContainer {
        position: absolute;
        width: 100vw;
        height: 100vh;
        top: 0;
        left: 0;
        background-color: #00000088;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .popup {
        padding: 10px;
        background-color: var(--dark-background);
    }

    .codeArgumentInput {
      border: none;
      outline: none;
      font-size: 18px;
      padding: 0 5px;
      margin: 2px 5px;
      background: transparent;
      color: white;
      /* Make it fill the rest of the flexbox */
      flex: 1;
      width: 0;
    }
    .codeArgumentInput::-webkit-inner-spin-button {
      -webkit-appearance: none;
      appearance: none;
    }
    .codeArgument {
      width: 230px;
      display: inline-flex;
      flex-direction: row;
      background-color: var(--primary-background);
      margin: 5px;
    }
    .codeArgument span {
      font-size: 18px;
      color: white;
      padding: 5px;
    }
</style>