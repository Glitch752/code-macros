<script setup>
  import { toRefs, ref } from "vue";

  const props = defineProps(["min", "max", "step", "defaultValue1", "defaultValue2"]);
  const emit = defineEmits(["change"]);

  const { min, max, step, defaultValue1, defaultValue2 } = toRefs(props);

  let value1 = ref(defaultValue1.value);
  let value2 = ref(defaultValue2.value);

  function slider1Input(e) {
    value1.value = Math.round(Math.min(value1.value, value2.value - step.value) * 10) / 10;
    emit("change", { value1: value1.value, value2: value2.value });
  }
  function slider2Input(e) {
    value2.value = Math.round(Math.max(value2.value, value1.value - (-step.value)) * 10) / 10;
    emit("change", { value1: value1.value, value2: value2.value });
  }

  function toPercent(value, min, max) {
    return (100 / (parseFloat(max) - parseFloat(min))) * parseFloat(value) - (100 / (parseFloat(max) - parseFloat(min))) * parseFloat(min);
  }
</script>

<template>
  <div class="container">
    <div class="slider">
      <div>
        <div class="inverse-left" :style="`width: ${toPercent(value1, min, max)}%`"></div>
        <div class="inverse-right" :style="`width: ${100 - toPercent(value1, min, max)}%`"></div>
        <div class="range" :style="`left: ${toPercent(value1, min, max)}%; right: ${100 - toPercent(value2, min, max)}%`"></div>
        <span class="thumb" :style="`left: ${toPercent(value1, min, max)}%`"></span>
        <span class="thumb" :style="`left: ${toPercent(value2, min, max)}%`"></span>
        <div class="sign" :style="`left: ${toPercent(value1, min, max)}%`">
          <span id="value">{{value1}}</span>
        </div>
        <div class="sign" :style="`left: ${toPercent(value2, min, max)}%`">
          <span id="value">{{value2}}</span>
        </div>
      </div>
      <input class="input" type="range" v-model="value1" :max="max" :min="min" :step="step" @input="slider1Input" />
      <input class="input" type="range" v-model="value2" :max="max" :min="min" :step="step" @input="slider2Input" />
    </div>
  </div>
</template>

<style scoped>
  .container {
    --accent: #477ba5;
    --track: #2e3544cc;
    --thumb: #2d4657;
    position: absolute;
    top: -40px;
    margin-left: -28px;
    height: 0px !important;
  }
  .slider {
    position: relative;
    height: 14px;
    border-radius: 10px;
    text-align: left;
    margin: 45px 0 10px 0;
    display: inline-block;
    width: 275px;
  }

  .slider > div {
    position: absolute;
    left: 13px;
    right: 15px;
    height: 14px;
  }

  .slider > div > .inverse-left {
    position: absolute;
    left: 0;
    height: 14px;
    background-color: var(--track);
    margin: 0 7px;
  }

  .slider > div > .inverse-right {
    position: absolute;
    right: 0;
    height: 14px;
    background-color: var(--track);
    margin: 0 7px;
  }

  .slider > div > .range {
    position: absolute;
    left: 0;
    height: 14px;
    background-color: var(--accent);
  }

  .slider > div > .thumb {
    position: absolute;
    top: -3px;
    z-index: 2;
    height: 20px;
    width: 20px;
    text-align: left;
    margin-left: -11px;
    cursor: pointer;
    box-shadow: 0 3px 8px rgba(0, 0, 0, 0.4);
    background-color: var(--thumb);
    outline: none;
  }

  .slider > .input {
    position: absolute;
    pointer-events: none;
    z-index: 3;
    height: 14px;
    top: -2px;
    width: 100%;
    opacity: 0;
  }

  div.slider > .input:focus::-webkit-slider-runnable-track {
    background: transparent;
    border: transparent;
  }

  div.slider > .input:focus {
    outline: none;
  }

  div.slider > .input::-webkit-slider-thumb {
    pointer-events: all;
    width: 28px;
    height: 28px;
    border: 0 none;
    background: red;
  }

  .slider > div > .sign {
    position: absolute;
    margin-left: -15px;
    top: -28px;
    z-index: 1;
    background-color: var(--accent);
    color: #fff;
    width: 30px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    opacity: var(--opacity, 1);
    pointer-events: none;
  }

  .slider > div > .sign > span {
    font-size: 12px;
    font-weight: 700;
  }
</style>