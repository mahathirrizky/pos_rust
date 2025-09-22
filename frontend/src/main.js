import { createApp } from 'vue';
import App from './App.vue';
import router from './router'; // Import the new router

// PrimeVue
import PrimeVue from 'primevue/config';
import Lara from '@primeuix/themes/lara';
import { definePreset } from '@primeuix/themes';
import Tooltip from 'primevue/tooltip';
import ToastService from 'primevue/toastservice';

// Styles
import 'primeicons/primeicons.css';
import './style.css';

// Define a new preset with green as the primary color
const MyPreset = definePreset(Lara, {
  semantic: {
    primary: {
      50: '{green.50}',
      100: '{green.100}',
      200: '{green.200}',
      300: '{green.300}',
      400: '{green.400}',
      500: '{green.500}',
      600: '{green.600}',
      700: '{green.700}',
      800: '{green.800}',
      900: '{green.900}',
      950: '{green.950}',
    },
  },
});

// Create and mount the Vue app
const app = createApp(App);

// Use PrimeVue with the custom green theme
app.use(PrimeVue, {
    theme: {
        preset: MyPreset,
        options: {
            darkModeSelector: 'disable-dark-mode',
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

// Use the router
app.use(router);

app.mount('#app');

 