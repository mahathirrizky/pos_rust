import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth';

export const useDashboardStore = defineStore('dashboard', {
  state: () => ({
    // Inventory Manager Dashboard Data
    products: [],
    suppliers: [],
    categories: [],
    inventoryReport: null,

    // Admin Dashboard Data
    adminSummaryData: null,
    adminRecentActivities: [],
    adminSalesChartData: null,

    isLoading: false,
    error: null,
  }),
  actions: {
    async fetchDashboardData(role = 'inventoryManager') {
      this.isLoading = true;
      this.error = null;
      const authStore = useAuthStore();
      if (!authStore.token) {
        this.error = new Error('No token found, user is not authenticated');
        console.error(this.error.message);
        this.isLoading = false;
        return;
      }
      const headers = { Authorization: `Bearer ${authStore.token}` };

      try {
        if (role === 'admin') {
          const [summaryRes, activitiesRes, salesRes] = await Promise.all([
            axios.get('/api/admin/dashboard/summary', { headers }),
            axios.get('/api/admin/dashboard/activities', { headers }),
            axios.get('/api/admin/dashboard/sales', { headers }),
          ]);
          this.adminSummaryData = summaryRes.data;
          this.adminRecentActivities = activitiesRes.data;
          this.adminSalesChartData = salesRes.data;
        } else if (role === 'inventoryManager') {
          const [productsRes, suppliersRes, categoriesRes, inventoryReportRes] = await Promise.all([
            axios.get('/api/products', { headers }),
            axios.get('/api/suppliers', { headers }),
            axios.get('/api/categories', { headers }),
            axios.get('/api/inventory/report', { headers }),
          ]);
          this.products = productsRes.data.data;
          this.suppliers = suppliersRes.data.data;
          this.categories = categoriesRes.data.data;
          this.inventoryReport = inventoryReportRes.data.data;
        }
      } catch (error) {
        this.error = error;
        console.error('Error fetching dashboard data:', error);
      } finally {
        this.isLoading = false;
      }
    },
  },
});
