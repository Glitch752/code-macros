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

  <div class="setting">
    <span>Theme</span>
    <span class="dropdown">
      <span></span>
      <select :value="theme" @change="selectTheme">
        <option value="darkTheme">Dark theme</option>
        <option value="lightTheme">Light theme</option>
      </select>
    </span>
  </div>

  <span class="backButton" @click="goBack">&lt;</span>
</template>

<style scoped>
  .setting {
    text-align: center;
    margin-top: 5px;
  }
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
  .dropdown {
    position: relative;
    margin-left: 10px;
  }
  .dropdown select {
    appearance: none;
    background-color: var(--dark-background);
    outline: none;
    border: none;
    border-radius: 0;
    padding: 5px 25px 5px 10px;
    color: var(--primary-text);
    font-size: 14px;
  }
  .dropdown span {
    width: 0; 
    height: 0; 
    border-left: 6px solid transparent;
    border-right: 6px solid transparent;
    border-top: 6px solid var(--primary-text);
    position: absolute;
    top: 10px;
    right: 10px;
  }
</style>