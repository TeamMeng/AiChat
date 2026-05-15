import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import store from './store';
import { initializeConfig } from './utils';

// import './assets/styles.css'; // Import any global styles
import './tailwind.css';

const bootstrap = async () => {
  await initializeConfig();

  const app = createApp(App);

  // Load user state from localStorage when the app starts
  store.dispatch('loadUserState');
  store.dispatch('appStart');

  app.use(store);
  app.use(router);

  app.mount('#app');
};

bootstrap();
