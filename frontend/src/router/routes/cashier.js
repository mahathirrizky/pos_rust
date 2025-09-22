import CashierLayout from '../../components/layout/CashierLayout.vue';
import CashierView from '../../views/cashier/CashierView.vue';
import BillsView from '../../views/placeholder/BillsView.vue';
import SettingsView from '../../views/placeholder/SettingsView.vue';

const cashierRoutes = {
  path: '/cashier',
  component: CashierLayout,
  children: [
    { path: '', name: 'CashierDashboard', component: CashierView, meta: { requiresAuth: true, allowedRoles: ['Cashier', 'Admin', 'StoreManager', 'InventoryManager'] } },
    {
      path: 'bills',
      name: 'CashierBills',
      component: BillsView,
      meta: { requiresAuth: true, allowedRoles: ['Cashier', 'Admin', 'StoreManager'] }
    },
    {
      path: 'settings',
      name: 'CashierSettings',
      component: SettingsView,
      meta: { requiresAuth: true, allowedRoles: ['Cashier', 'Admin', 'StoreManager', 'InventoryManager'] }
    },
  ]
};

export default cashierRoutes;