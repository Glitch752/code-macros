<script setup>
  import { useRouter } from "vue-router";

  let router = useRouter();

  function goBack() {
    router.push("/macros");
  }
</script>

<template>
  <h1>Settings</h1>
  <span class="backButton" @click="goBack">&lt;</span>
</template>

<style scoped>
  h1 {
    font-size: 30px;
    margin: 0;
    font-weight: 500;
    text-align: center;
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
</style>