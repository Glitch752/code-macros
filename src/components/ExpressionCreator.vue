<script setup>
    import { toRefs } from 'vue';
  
    import ExpressionCreator from './ExpressionCreator.vue';
  
    import expressionTypes from '../data/expressionTypes.json';
  
    const props = defineProps(["expression"]);
  
    const { expression } = toRefs(props);
  
    function setExpression(e) {
      let expressionType = expressionTypes.find(type => type.value === e.target.value);
      expression.value = {};
      expression.value.type = expressionType.value;
      for(let key in expressionType.defaultData) {
          let value = expressionType.defaultData[key];
          expression.value[key] = value;
      }
    }
</script>

<template>
    <div class="expression">
        <span class="expressionSelectContainer">
            <span class="expressionSelectArrow"></span>
            <select class="expressionSelect" :value="expression.type" @change="setExpression">
                <option 
                    :value="expressionType.value"
                    v-for="expressionType in expressionTypes" :key="expressionType.value">{{expressionType.name}}</option>
            </select>
        </span>
        <span class="expressionBoolean" v-if="expression.type === 'number'">
            <input class="expressionInput" v-model="expression.value" placeholder="Number" type="number" />
        </span>
        <span class="expressionBoolean" v-if="expression.type === 'variable'">
            <input class="expressionInput" v-model="expression.variable" placeholder="Variable" type="text" />
        </span>
        <span class="expressionBoolean" v-if="expression.type === 'arithmetic'">
            <ExpressionCreator :expression="expression.left"/>
            <span class="expressionSelectContainer">
                <span class="expressionSelectArrow"></span>
                <select class="expressionSelect" :value="expression.kind" @change="(e) => expression.kind = e.target.value">
                    <option value="addition"       >Addition       </option>
                    <option value="subtraction"    >Subtraction    </option>
                    <option value="division"       >Division       </option>
                    <option value="multiplication" >Multiplication </option>
                    <option value="modulo"         >Modulo         </option>
                    <option value="exponent"       >Exponent       </option>
                </select>
            </span>
            <ExpressionCreator :expression="expression.right" />
        </span>
        <span class="expressionBoolean" v-if="expression.type === 'bitwise'">
            <ExpressionCreator :expression="expression.left"/>
            <span class="expressionSelectContainer">
                <span class="expressionSelectArrow"></span>
                <select class="expressionSelect" :value="expression.kind" @change="(e) => expression.kind = e.target.value">
                    <option value="and">And</option>
                    <option value="or">Or</option>
                    <option value="xor">Xor</option>
                    <option value="not">Not</option>
                    <option value="leftshift">Left shift</option>
                    <option value="signrightshift">Right shift</option>
                </select>
            </span>
            <ExpressionCreator :expression="expression.right" v-if="expression.kind !== 'not'" />
        </span>
    </div>
</template>

<style scoped>
    .expression {
        background-color: var(--primary-background);
        margin: 5px;
        padding: 5px;
    }
    .expressionSelect {
        appearance: none;
        background-color: var(--dark-background);
        outline: none;
        border: none;
        border-radius: 0;
        padding: 5px 25px 5px 10px;
        color: #fff;
        font-size: 14px;
    }
    .expressionSelectArrow {
        width: 0; 
        height: 0; 
        border-left: 6px solid transparent;
        border-right: 6px solid transparent;
        
        border-top: 6px solid #fff;

        position: absolute;
        top: 10px;
        right: 10px;
    }
    .expressionSelectContainer {
        position: relative;
    }
    .expressionInput {
        background-color: var(--dark-background);
        outline: none;
        border: none;
        padding: 5px;
        color: #fff;
    }
    .expressionInput::-webkit-inner-spin-button {
        appearance: none;
    }
</style>