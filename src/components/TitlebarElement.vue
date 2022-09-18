<script setup>
  import { appWindow } from '@tauri-apps/api/window';
  import { onMounted } from 'vue';
  onMounted(() => {
    document.getElementById('titlebar-minimize').addEventListener('click', () => appWindow.minimize());
    document.getElementById('titlebar-maximize-unmaximize').addEventListener('click', () => appWindow.toggleMaximize());
    document.getElementById('titlebar-close').addEventListener('click', () => appWindow.hide());
    window.addEventListener('resize', updateWindow)
    updateWindow();
  });
  function updateWindow() {
    let app = document.getElementById("app");
    appWindow.isMaximized().then(maximized => {
      app.classList.toggle('maximized', maximized);
    });
  }
</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-name">
      Macro Creator
    </div>
    <div class="titlebar-button" id="titlebar-minimize">
      <img
      src="https://api.iconify.design/mdi:window-minimize.svg"
      alt="minimize"
      />
    </div>
    <div class="titlebar-button" id="titlebar-maximize-unmaximize">
      <div id="titlebar-maximize">
        <img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize"
        />
      </div>
      <div id="titlebar-unmaximize">
        <img
        src="https://api.iconify.design/mdi:window-restore.svg"
        alt="unmaximize"
        />
      </div>
    </div>
    <div class="titlebar-button" id="titlebar-close">
      <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
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
    color: white;
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