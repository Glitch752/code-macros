import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import updateMacros from './utils';

import { loadTheme } from "@/utils";

loadTheme();

createApp(App).use(router).mount('#app');

updateMacros();