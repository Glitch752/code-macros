<script setup>
  import { ref } from 'vue';
  import { useRouter } from "vue-router";
  import * as store from '@/store';
  import { selectTheme } from "@/utils";

  let router = useRouter();

  let theme = ref(null);

  store.get("theme", "darkTheme").then((value) => {
    theme.value = value;
  });

  function goBack() {
    router.push("/macros");
  }
</script>

<template>
  <h1>Settings</h1>

  <span>Theme</span>
  <select :value="theme" @change="selectTheme">
    <option value="darkTheme">Dark theme</option>
    <option value="lightTheme">Light theme</option>
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