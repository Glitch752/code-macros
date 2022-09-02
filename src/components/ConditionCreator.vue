<script setup>
    import { toRefs } from 'vue';

    import ConditionCreator from './ConditionCreator.vue';
    import ExpressionCreator from './ExpressionCreator.vue';

    import conditionTypes from '../data/conditionTypes.json';

    const props = defineProps(["condition"]);

    const { condition } = toRefs(props);

    function setCondition(e) {
        let conditionType = conditionTypes.find(type => type.value === e.target.value);
        condition.value = {};
        condition.value.type = conditionType.value;
        for(let key in conditionType.defaultData) {
            let value = conditionType.defaultData[key];
            condition.value[key] = value;
        }
    }
</script>

<template>
    <div class="condition">
        <span class="conditionSelectContainer">
            <span class="conditionSelectArrow"></span>
            <select class="conditionSelect" :value="condition.type" @change="setCondition">
                <option 
                    :value="conditionType.value"
                    v-for="conditionType in conditionTypes" :key="conditionType.value">{{conditionType.name}}</option>
            </select>
        </span>
        <span class="conditionBoolean" v-if="condition.type === 'boolean'">
            <span class="conditionSelectContainer">
                <span class="conditionSelectArrow"></span>
                <select class="conditionSelect" :value="condition.value" @change="(e) => condition.value = e.target.value === 'true' ? true : false">
                    <option value="false">False</option>
                    <option value="true">True</option>
                </select>
            </span>
        </span>
        <span class="conditionBoolean" v-if="condition.type === 'number'">
            <input class="conditionInput" v-model="condition.value" placeholder="Number" type="number" />
        </span>
        <span class="conditionBoolean" v-if="condition.type === 'comparison'">
            <ConditionCreator :condition="condition.left" />
            <span class="conditionSelectContainer">
                <span class="conditionSelectArrow"></span>
                <select class="conditionSelect" :value="condition.comparison" @change="(e) => condition.comparison = e.target.value">
                    <option value=">">&gt;</option>
                    <option value="<">&lt;</option>
                    <option value=">=">&gt;=</option>
                    <option value="<=">&lt;=</option>
                    <option value="==">==</option>
                    <option value="!==">!==</option>
                </select>
            </span>
            <ConditionCreator :condition="condition.right" />
        </span>
        <span class="conditionBoolean" v-if="condition.type === 'logical'">
            <ConditionCreator :condition="condition.left" v-if="condition.kind !== 'not'"/>
            <span class="conditionSelectContainer">
                <span class="conditionSelectArrow"></span>
                <select class="conditionSelect" :value="condition.kind" @change="(e) => condition.kind = e.target.value">
                    <option value="and">And</option>
                    <option value="or">Or</option>
                    <option value="not">Not</option>
                </select>
            </span>
            <ConditionCreator :condition="condition.right" />
        </span>
        <span class="conditionBoolean" v-if="condition.type === 'variable'">
            <input class="conditionInput" v-model="condition.variable" placeholder="Variable" type="text" />
        </span>
        <span class="conditionBoolean" v-if="condition.type === 'expression'">
            <ExpressionCreator :expression="condition.expression" />
        </span>
    </div>
</template>

<style scoped>
    .condition {
        border: 3px solid #141a2766;
        margin: 5px;
        padding: 5px;
    }
    .conditionSelect {
        appearance: none;
        background-color: transparent;
        outline: none;
        border: 3px solid #141a2766;
        border-radius: 0;
        padding: 5px 25px 5px 10px;
        color: #fff;
        font-size: 14px;
    }
    .conditionSelectArrow {
        width: 0; 
        height: 0; 
        border-left: 6px solid transparent;
        border-right: 6px solid transparent;
        
        border-top: 6px solid #fff;

        position: absolute;
        top: 10px;
        right: 10px;
    }
    .conditionSelectContainer {
        position: relative;
    }
    .conditionInput {
        background-color: transparent;
        outline: none;
        border: 3px solid #141a2766;
        padding: 5px;
        color: #fff;
    }
    .conditionInput::-webkit-inner-spin-button {
        appearance: none;
    }
</style>