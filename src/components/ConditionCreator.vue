<script setup>
  import { toRefs } from 'vue';

  const props = defineProps(["condition"]);

  const { condition } = toRefs(props);

  console.log(condition.value.value);
</script>

<template>
    <div class="condition">
        <span class="conditionBoolean" v-if="condition.type === 'boolean'">
            <span class="conditionSelectArrow"></span>
            <select class="conditionSelect" :value="condition.value" @change="(e) => condition.value = e.target.value === 'true' ? true : false">
                <option value="false">False</option>
                <option value="true">True</option>
            </select>
            <span style="position: absolute; top: 0; margin-left: 5px;">(boolean)</span>
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
    .conditionBoolean {
        position: relative;
    }
</style>