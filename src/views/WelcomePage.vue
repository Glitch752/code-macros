<template>
  <div>
    <h1>Welcome to this unnamed macro creator!</h1>
    <p>This software allows you to visually create macros that can do bascially anything you want!</p>
    <p>You <s>can</s> will eventually be able to create macros that do things like:</p>
    <ul>
      <li>Send a notification after a certain key combination</li>
      <li>Open another application a random amount of seconds after you open a different one</li>
      <li>And more</li>
    </ul>
    <p>This is very much a work in progress!</p>
    <router-link to="/macros" class="getStarted">Get started</router-link>
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
    font-size: 1.5em;
    font-weight: bold;
    color: white;
    text-decoration: none;
    background-color: #2c4d94;
    padding: 5px 15px;
    margin: 10px;
    cursor: pointer;
    display: inline-block;
    transform: translate(0, 0);
    box-shadow: 3px 3px #233e77;
  }
  .getStarted:hover {
    background-color: #2f54a3;
    transform: translate(-1px, -1px);
    box-shadow: 4px 4px #2a498d;
  }
  .getStarted:active {
    background-color: #23407e;
    transform: translate(1px, 1px);
    box-shadow: 2px 2px #1c315f;
  }
</style>