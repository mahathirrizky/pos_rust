import StoreManagerLayout from '../../components/layout/StoreManagerLayout.vue';

const storeManagerRoutes = [
  {
    path: '/store-manager',
    component: StoreManagerLayout,
    meta: { allowedRoles: ['StoreManager', 'Admin', 'Owner'] },
    children: [
      {
        path: '',
        name: 'store-manager-dashboard',
        component: () => import('../../views/storeManager/Dashboard.vue'),
      },
      {
        path: 'inventory',
        name: 'store-manager-inventory',
        component: () => import('../../views/storeManager/Inventory.vue'),
      },
      {
        path: 'products',
        name: 'store-manager-products',
        component: () => import('../../views/storeManager/Products.vue'),
      },
      {
        path: 'suppliers',
        name: 'store-manager-suppliers',
        component: () => import('../../views/storeManager/Suppliers.vue'),
      },
      {
        path: 'reports',
        name: 'store-manager-reports',
        component: () => import('../../views/storeManager/Reports.vue'),
      },
      {
        path: 'employees',
        name: 'store-manager-employees',
        component: () => import('../../views/storeManager/Employees.vue'),
      },
      {
        path: 'promotions',
        name: 'store-manager-promotions',
        component: () => import('../../views/storeManager/Promotions.vue'),
      },
      {
        path: 'refunds',
        name: 'store-manager-refunds',
        component: () => import('../../views/storeManager/Refunds.vue'),
      },
      {
        path: 'settings',
        name: 'store-manager-settings',
        component: () => import('../../views/storeManager/Settings.vue'),
      },
    ],
  },
];

export default storeManagerRoutes;