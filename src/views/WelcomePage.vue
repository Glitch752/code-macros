<template>
  <div>
    <h1>Welcome to this unnamed macro creator!</h1>
    <p>This software allows you to visually create macros that can do bascially anything you want!</p>
    <p>You <s>can</s> will eventually be able to create macros that do things like:</p>
    <ul>
      <li>Send a notification after a certain key combination</li>
      <li>Press a set of keys every half second</li>
      <li>And more</li>
    </ul>
    <p>This is very much a work in progress!</p>
    <router-link to="/macros" class="getStarted button">Get started</router-link>
  </div>
</template>

<style scoped>
  div {
      text-align: center;
      padding: 20px;
  }
  ul {
      list-style: inside;
      padding: 0;
      margin: 0;
  }
  .getStarted {
    --defaultBG: #2c4d94;
    --defaultBoxBG: #233e77;
    --hoverBG: #2f54a3;
    --hoverBoxBG: #2a498d;
    --activeBG: #23407e;
    --activeBoxBG: #1c315f;
  }
</style>