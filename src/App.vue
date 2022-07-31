<script setup>
  import TitlebarElement from '@/components/TitlebarElement.vue';
</script>

<template>
  <TitlebarElement />
  <router-view />
</template>

<style>
  *, *::before, *::after {
    box-sizing: border-box;
  }
  #app {
    --inset: 0;
    position: absolute;
    inset: var(--inset);
    background-color: #1c2433dd;
    color: white;
    font-family: 'Open Sans';
    border: 3px solid #30467777;
  }
  #app.maximized {
    border: none;
    --inset: 0;
  }
  body {
    overflow: hidden;
  }
</style>