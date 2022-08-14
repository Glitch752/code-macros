import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import { invoke } from '@tauri-apps/api/tauri';
import * as store from './store';

createApp(App).use(router).mount('#app');

store.get('macros', []).then((data) => {
    invoke('update_macros', { macros: JSON.stringify(data) });
});