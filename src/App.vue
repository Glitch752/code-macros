<script setup>
  import TitlebarElement from '@/components/TitlebarElement.vue';
</script>

<template>
  <TitlebarElement />
  <div class="relative">
    <router-view />
  </div>
</template>

<style>
  /* :root { */
    /* Dark theme */
    /* --primary-background: #1e1f29;
    --dark-background: #171820;
    --secondary-background: #20242e;
    --primary-text: #ffffff;
    --secondary-text: #dddddd; */

    /* Light theme */
    /* --primary-background: #e4e4e4;
    --dark-background: #bebfc2;
    --secondary-background: #f5f5f5;
    --primary-text: #000000;
    --secondary-text: #1d1d1d; */
  /* } */
  
  /* Dark theme */
  body.darkTheme {
    --primary-background: #1e1f29;
    --dark-background: #171820;
    --secondary-background: #20242e;
    --primary-text: #ffffff;
    --secondary-text: #dddddd;

    --accent: #242935;
    --accent-light: #292f3d;
  }
  /* Light theme */
  body.lightTheme {
    --primary-background: #e4e4e4;
    --dark-background: #bebfc2;
    --secondary-background: #f5f5f5;
    --primary-text: #000000;
    --secondary-text: #1d1d1d;
    
    --accent: #214a80;
    --accent-light: #376aad;
  }
  *, *::before, *::after {
    box-sizing: border-box;
  }
  *:not(input, textarea) {
    user-select: none;
  }
  .relative {
    position: relative;
    top: 24px;
    height: 100%;
  }
  #app {
    --inset: 0;
    position: absolute;
    inset: var(--inset);
    background-color: var(--primary-background);
    color: var(--primary-text);
    font-family: 'Source Sans Pro';
  }
  #app.maximized {
    border: none;
    --inset: 0;
  }
  body {
    overflow: hidden;
  }
  .button {
    font-size: 1.5em;
    font-weight: 500;
    color: var(--primary-text);
    text-decoration: none;
    background-color: var(--defaultBG);
    padding: 5px 15px;
    margin: 10px;
    cursor: pointer;
    display: inline-block;
    transform: translate(0, 0);
    box-shadow: 3px 3px var(--defaultBoxBG);
    border: none;
    outline: none;
  }
  .button:hover {
    background-color: var(--hoverBG);
    transform: translate(-1px, -1px);
    box-shadow: 4px 4px var(--hoverBoxBG);
  }
  .button:active {
    background-color: var(--activeBG);
    transform: translate(1px, 1px);
    box-shadow: 2px 2px var(--activeBoxBG);
  }

  ::-webkit-scrollbar {
    width: 10px;
  }
  ::-webkit-scrollbar-track {
    background: var(--dark-background); 
  }
  ::-webkit-scrollbar-thumb {
    background: var(--primary-background); 
  }
  ::-webkit-scrollbar-thumb:hover {
    background: #252733; 
  }

  .insertPlaceholder {
    margin: 0 10px 0 10px;
    height: 5px;
    background-color: #4276b1;
    border-radius: 100vw;
  }
  
  h1 {
    font-size: 30px;
    margin: 0;
    font-weight: 500;
    text-align: center;
    color: var(--primary-text);
  }
</style>