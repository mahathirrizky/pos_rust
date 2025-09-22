import { createRouter, createWebHistory } from 'vue-router';
import CashierLayout from '../components/layout/CashierLayout.vue';
import CashierView from '../views/cashier/CashierView.vue';
import BillsView from '../views/placeholder/BillsView.vue';
import SettingsView from '../views/placeholder/SettingsView.vue';

const routes = [
  {
    path: '/',
    component: CashierLayout,
    children: [
      { path: '', redirect: '/cashier' },
      { path: 'cashier', name: 'Cashier', component: CashierView },
      { path: 'bills', name: 'Bills', component: BillsView },
      { path: 'settings', name: 'Settings', component: SettingsView },
    ]
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
