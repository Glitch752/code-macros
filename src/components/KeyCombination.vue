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

  function shiftToLower(key) {
    let lowercaseKey = key.toLowerCase();
    if(lowercaseKey === key) {
      const specialCharacters = {
        "\"": "'",
        "~": "`",
        "!": "1",
        "@": "2",
        "#": "3",
        "$": "4",
        "%": "5",
        "^": "6",
        "&": "7",
        "*": "8",
        "(": "9",
        ")": "0",
        "?": "/",
        "<": ",",
        ">": ".",
        ":": ";",
        "|": "\\",
        " ": "space",
        "{": "[",
        "}": "]",
        "+": "=",
        "_": "-"
      }
      lowercaseKey = specialCharacters[key] || lowercaseKey;
      return lowercaseKey;
    } else {
      return lowercaseKey;
    }
  }

  function keyAllowed(key) {
    const keysAllowed = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
                         "shift", "control", "enter", "space", "escape", "tab", "capslock", "backspace", ";", "'", ",", ".", "/", "[", "]", "`", "-", "=",
                         "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10"];
    
    return keysAllowed.includes(key);
  }

  function keyDown(e) {
    let key = shiftToLower(e.key);
    if(!keyAllowed(key)) return;
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
    let key = shiftToLower(e.key);
    if(!keyAllowed(key)) return;
    setTimeout(() => {
      keysHeld[key] = false;
    }, 150);
  }
</script>

<template>
    <div class="selectOuter">
        <div class="selectInner" @click="setKeyCombination()">
            <span ref="keyCombinationElement" class="keyCombination">{{ initiator.data.keys.join(" + ") }}</span>
        </div>
        <span>Click to set key combination</span>
    </div>
</template>

<style scoped>
  .selectOuter {
    margin: 5px;
    padding: 5px;
    width: 100%;
    background-color: var(--primary-background);
  }
  .selectInner {
    padding: 5px;
    width: 100%;
    background-color: var(--secondary-background);
  }
  .selectOuter span {
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