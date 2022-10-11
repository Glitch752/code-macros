<script setup>
  import { useRouter } from "vue-router";

  let router = useRouter();

  function goBack() {
    router.push("/macros");
  }

  function selectTheme(e) {
    // TODO: Make this save in config.json
    document.body.className = "";
    const themeToClass = {
      "dark": "darkTheme",
      "light": "lightTheme"
    }
    document.body.classList.add(themeToClass[e.target.value]);
  }
</script>

<template>
  <h1>Settings</h1>

  <span>Theme</span>
  <select @change="selectTheme">
    <option value="dark">Dark theme</option>
    <option value="light">Light theme</option>
  </select>

  <span class="backButton" @click="goBack">&lt;</span>
</template>

<style scoped>
  .backButton {
    position: absolute;
    top: -10px;
    left: 10px;
    font-size: 50px;
    font-weight: 800;
    color: var(--secondary-text);
    cursor: pointer;
  }
  .backButton:hover {
    color: var(--primary-text);
  }
</style>