<script setup>
  import { onUnmounted, toRefs, ref } from "vue";
  let listeningForKeyCombination = false;

  const props = defineProps(["initiator"]);

  const { initiator } = toRefs(props);

  const keyCombinationElement = ref(null);

  function setKeyCombination() {
    keyCombinationElement.value.textContent = "Listening...";
    listeningForKeyCombination = true;
  }

  window.addEventListener("keydown", keyDown);
  window.addEventListener("keyup", keyUp);

  onUnmounted(() => {
    window.removeEventListener("keydown", keyDown);
    window.removeEventListener("keyup", keyUp);
  });

  let keysHeld = {};

  let callbackTimeout = null;
  function debounce(callback, time) {
    if (callbackTimeout) clearTimeout(callbackTimeout);
    callbackTimeout = setTimeout(callback, time);
  }

  const keysWithValue = (object, value) => Object.keys(object).filter(key => object[key] === value);

  function keyDown(e) {
    let key = e.key.toLowerCase();
    keysHeld[key] = true;
    if(!listeningForKeyCombination) return;
    debounce(() => {
      if (keysWithValue(keysHeld, true).length > 0) {
        keyCombinationElement.value.textContent = keysWithValue(keysHeld, true).join(" + ");
        listeningForKeyCombination = false;
        initiator.value.data.keys = keysWithValue(keysHeld, true);
      }
    }, 150);
  }
  function keyUp(e) {
    let key = e.key.toLowerCase();
    setTimeout(() => {
      keysHeld[key] = false;
    }, 150);
  }
</script>

<template>
    <div class="selectInner">
        <div class="selectInner" @click="setKeyCombination()">
            <span ref="keyCombinationElement" class="keyCombination">{{ initiator.data.keys.join(" + ") }}</span>
        </div>
        <span>Click to set key combination</span>
    </div>
</template>

<style scoped>
  .selectInner {
    margin: 5px;
    padding: 5px;
    width: 100%;
    background-color: #1f212799;
  }
  .selectInner span {
    font-size: 16px;
    text-align: center;
    display: inline-block;
    width: 100%;
    color: #ddd;
  }
  .keyCombination {
    font-size: 40px !important;
    font-weight: bold;
    color: #fff;
  }
</style>