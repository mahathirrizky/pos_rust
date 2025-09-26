import CashierLayout from '../../components/layout/CashierLayout.vue';

const cashierRoutes = {
  path: '/cashier',
  component: CashierLayout,
  meta: { allowedRoles: ['Cashier', 'Admin', 'Owner'] },
  children: [
    { path: '', name: 'CashierDashboard', component: () => import('../../views/cashier/CashierView.vue'),  },
    {
      path: 'bills',
      name: 'CashierBills',
      component: () => import('../../views/cashier/Bills.vue'),
      
    },
    {
      path: 'settings',
      name: 'CashierSettings',
      component: () => import('../../views/cashier/Settings.vue'),
      
    },
  ]
};

export default cashierRoutes;