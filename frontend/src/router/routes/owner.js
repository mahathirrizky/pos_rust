import OwnerLayout from '../../components/layout/OwnerLayout.vue';

const ownerRoutes = [
  {
    path: '/owner',
    component: OwnerLayout,
    meta: { allowedRoles: ['Owner'] },
    children: [
      {
        path: '',
        name: 'owner-dashboard',
        component: () => import('../../views/owner/Dashboard.vue'),

      },
      {
        path: 'stores',
        name: 'owner-stores',
        component: () => import('../../views/owner/Stores.vue'),

      },
      {
        path: 'admin-users',
        name: 'owner-admin-users',
        component: () => import('../../views/owner/AdminUsers.vue'),

      },
      {
        path: 'bills',
        name: 'owner-bills',
        component: () => import('../../views/owner/Bills.vue'),

      },
      {
        path: 'settings',
        name: 'owner-settings',
        component: () => import('../../views/owner/Settings.vue'),

      },
      {
        path: 'inventory',
        name: 'owner-inventory',
        component: () => import('../../views/owner/Inventory.vue'),

      },
      {
        path: 'products',
        name: 'owner-products',
        component: () => import('../../views/owner/Products.vue'),

      },
      {
        path: 'suppliers',
        name: 'owner-suppliers',
        component: () => import('../../views/owner/Suppliers.vue'),

      },
      {
        path: 'reports',
        name: 'owner-reports',
        component: () => import('../../views/owner/Reports.vue'),

      },
      {
        path: 'employees',
        name: 'owner-employees',
        component: () => import('../../views/owner/Employees.vue'),

      },
      {
        path: 'purchase-orders',
        name: 'owner-purchase-orders',
        component: () => import('../../views/purchase-orders/PurchaseOrderListView.vue'),
      },
      {
        path: 'purchase-orders/:id',
        name: 'owner-purchase-order-detail',
        component: () => import('../../views/purchase-orders/PurchaseOrderDetailView.vue'),
      },
      
      {
        path: 'refunds',
        name: 'owner-refunds',
        component: () => import('../../views/owner/Refunds.vue'),

      },
    ],
  },
];

export default ownerRoutes;