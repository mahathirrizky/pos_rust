import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth';

export const useDashboardStore = defineStore('dashboard', {
  state: () => ({
    products: [],
    suppliers: [],
    categories: [],
    inventoryReport: null,
  }),
  actions: {
    async fetchDashboardData() {
      const authStore = useAuthStore();
      if (!authStore.token) {
        console.error('No token found, user is not authenticated');
        return;
      }
      const headers = { Authorization: `Bearer ${authStore.token}` };
      try {
        const [productsRes, suppliersRes, categoriesRes, inventoryReportRes] = await Promise.all([
          axios.get('/api/products', { headers }),
          axios.get('/api/suppliers', { headers }),
          axios.get('/api/categories', { headers }),
          axios.get('/api/inventory/report', { headers }),
        ]);
        this.products = productsRes.data;
        this.suppliers = suppliersRes.data;
        this.categories = categoriesRes.data;
        this.inventoryReport = inventoryReportRes.data;
      } catch (error) {
        console.error('Error fetching dashboard data:', error);
      }
    },
  },
});