import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import router from './router'; // Import the new router
import axios from 'axios';

// PrimeVue
import PrimeVue from 'primevue/config';
import Lara from '@primeuix/themes/lara';
import { definePreset } from '@primeuix/themes';
import Tooltip from 'primevue/tooltip';
import ToastService from 'primevue/toastservice';
import ConfirmationService from 'primevue/confirmationservice';

// Styles
import 'primeicons/primeicons.css';
import './style.css';

// Define a new preset with green as the primary color
const MyPreset = definePreset(Lara, {
  semantic: {
    primary: {
      50: '{blue.50}',
      100: '{blue.100}',
      200: '{blue.200}',
      300: '{blue.300}',
      400: '{blue.400}',
      500: '{blue.500}',
      600: '{blue.600}',
      700: '{blue.700}',
      800: '{blue.800}',
      900: '{blue.900}',
      950: '{blue.950}',
    },
    surface: {
      0: '{bluegray.50}', // Light mode base background
      50: '{bluegray.100}',
      100: '{bluegray.200}',
      200: '{bluegray.300}',
      300: '{bluegray.400}',
      400: '{bluegray.500}',
      500: '{bluegray.600}',
      600: '{bluegray.700}',
      700: '{bluegray.800}',
      800: '{bluegray.900}', // Dark mode base background
      900: '{bluegray.950}',
      950: '{bluegray.950}',
    },
    text: {
      color: '{bluegray.800}',
      'color-secondary': '{bluegray.600}',
      'color-muted': '{bluegray.400}',
    },
    border: {
      color: '{bluegray.200}',
      'color-light': '{bluegray.100}',
    }
  },
});

// Create and mount the Vue app
const app = createApp(App);

// Use PrimeVue with the custom green theme
app.use(PrimeVue, {
    theme: {
        preset: MyPreset,
        options: {
            darkModeSelector: '.dark',
            cssLayer: {
              name: "primevue",
              order: "theme, base, primevue",
            },
        }
    }
});

// Register directives
app.directive('tooltip', Tooltip);

// Use ToastService
app.use(ToastService);
app.use(ConfirmationService);

// Use Pinia
app.use(createPinia());

// Use the router
app.use(router);

axios.defaults.baseURL = 'http://localhost:8000';

axios.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

app.mount('#app');

 