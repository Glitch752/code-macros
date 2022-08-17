import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import updateMacros from './utils';

createApp(App).use(router).mount('#app');

updateMacros();