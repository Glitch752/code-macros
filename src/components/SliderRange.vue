<script setup>
  import { toRefs, ref } from "vue";

  const props = defineProps(["min", "max", "step", "value1", "value2"]);
  const emit = defineEmits(["change"]);

  const { min, max, step, defaultValue1, defaultValue2 } = toRefs(props);

  let value1 = ref(defaultValue1);
  let value2 = ref(defaultValue2);

  function slider1Input(e) {
    let elem = e.target;
    elem.value = Math.min(elem.value, elem.parentNode.childNodes[2].value - 1);
    var value = (100 / (parseInt(elem.max) - parseInt(elem.min))) * parseInt(elem.value) - (100 / (parseInt(elem.max) - parseInt(elem.min))) * parseInt(elem.min);
    var children = elem.parentNode.childNodes[0].childNodes;
    children[0].style.width = value + '%';
    children[2].style.left = value + '%';
    children[3].style.left = value + '%';
    children[5].style.left = value + '%';
    value1.value = elem.value;
  }
  function slider2Input(e) {
    let elem = e.target;
    elem.value = Math.max(elem.value, elem.parentNode.childNodes[1].value - (-1));
    var value = (100 / (parseInt(elem.max) - parseInt(elem.min))) * parseInt(elem.value) - (100 / (parseInt(elem.max) - parseInt(elem.min))) * parseInt(elem.min);
    var children = elem.parentNode.childNodes[0].childNodes;
    children[1].style.width =  (100 - value) + '%';
    children[2].style.right = (100 - value) + '%';
    children[4].style.left = value + '%';
    children[6].style.left = value + '%';
    value2.value = elem.value;
  }
</script>

<template>
  <div class="container">
    <div class="slider" id="slider-distance">
      <div>
        <div class="inverse-left" style="width:70%;"></div>
        <div class="inverse-right" style="width:70%;"></div>
        <div class="range" style="left:30%;right:40%;"></div>
        <span class="thumb" style="left:30%;"></span>
        <span class="thumb" style="left:60%;"></span>
        <div class="sign" style="left:30%;">
          <span id="value">{{value1 || "30"}}</span>
        </div>
        <div class="sign" style="left:60%;">
          <span id="value">{{value2 || "60"}}</span>
      </div>
    </div>
    <input class="input" type="range" value="30" :max="max" :min="min" :step="step" @input="slider1Input" />
    <input class="input" type="range" value="60" :max="max" :min="min" :step="step" @input="slider2Input" />
    </div>
  </div>
</template>

<style scoped>
  .container {
    --accent: #47a54f;
    --track: #2e3544cc;
    --thumb: #2d5730;
    position: absolute;
    top: -20px;
    display: inline-block;
  }
  .slider {
    position: relative;
    height: 14px;
    border-radius: 10px;
    text-align: left;
    margin: 45px 0 10px 0;
    display: inline-block;
    width: 250px;
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
  }

  .slider > div > .sign > span {
    font-size: 12px;
    font-weight: 700;
  }
</style>