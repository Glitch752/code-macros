<script setup>
  import { appWindow } from '@tauri-apps/api/window';
  import { onMounted, ref } from 'vue';
  import { useRouter } from 'vue-router';

  let router = useRouter();

  onMounted(() => {
    window.addEventListener('resize', updateWindow);
    updateWindow();
  });
  function updateWindow() {
    let app = document.getElementById("app");
    appWindow.isMaximized().then(maximized => {
      app.classList.toggle('maximized', maximized);
    });
  }

  function openSettings() {
    router.push("/settings");
  }
  
  const color = ref(getComputedStyle(document.body).getPropertyValue("--primary-text").replace("#", "").trim());

  // TODO: Find a more Vue-like way of doing this
  document.themeChangedCallback = () => {
    color.value = getComputedStyle(document.body).getPropertyValue("--primary-text").replace("#", "").trim();
  }
</script>

<template>
  <div data-tauri-drag-region class="titlebar" :key="color">
    <div class="titlebar-name">
      Macro Creator
    </div>
    <div class="titlebar-button" @click="openSettings">
      <img :src="`https://api.iconify.design/mdi:cog.svg?color=%23${color}`" alt="settings" />
    </div>
    <div class="titlebar-button" @click="appWindow.minimize()">
      <img
      :src="`https://api.iconify.design/mdi:window-minimize.svg?color=%23${color}`"
      alt="minimize"
      />
    </div>
    <div class="titlebar-button" @click="appWindow.toggleMaximize()">
      <div id="titlebar-maximize">
        <img
        :src="`https://api.iconify.design/mdi:window-maximize.svg?color=%23${color}`"
        alt="maximize"
        />
      </div>
      <div id="titlebar-unmaximize">
        <img
        :src="`https://api.iconify.design/mdi:window-restore.svg?color=%23${color}`"
        alt="unmaximize"
        />
      </div>
    </div>
    <div class="titlebar-button" @click="appWindow.hide()">
      <img :src="`https://api.iconify.design/mdi:close.svg?color=%23${color}`" alt="close" />
    </div>
  </div>
</template>

<style scoped>
  .titlebar {
    height: 24px;
    background: var(--dark-background);
    user-select: none;
    display: flex;
    justify-content: flex-end;
    position: fixed;
    top: var(--inset);
    left: var(--inset);
    right: var(--inset);
    overflow: hidden;
  }
  .titlebar-name {
    text-align: center;
    color: var(--primary-text);
    font-size: 16px;
    position: absolute;
    left: 5px;
  }
  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 24px;
    height: 24px;
  }
  .titlebar-button:last-of-type {
    padding-right: var(--inset);
  }
  #titlebar-maximize {
    display: none;
  }
  #titlebar-unmaximize {
    display: block;
  }
  #app:not(.maximized) #titlebar-maximize {
    display: block;
  }
  #app:not(.maximized) #titlebar-unmaximize {
    display: none;
  }
  .titlebar-button:hover {
    background: #4d71bd55;
  }
</style>