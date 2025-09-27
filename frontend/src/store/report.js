import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth';

export const useReportStore = defineStore('report', {
  state: () => ({
    summaryData: null,
    isLoading: false,
    salesToday: null,
    salesWeek: null,
    salesMonth: null,
    salesYear: null,
    totalRevenue: 0,
    totalOrders: 0,
    newCustomers: 0,
    totalRefunds: 0,
    dailyRevenue: [],
  }),
  actions: {
    async fetchSalesReports() {
      const authStore = useAuthStore();
      if (!authStore.token) {
        console.error('No token found, user is not authenticated');
        return;
      }
      const headers = { Authorization: `Bearer ${authStore.token}` };
      try {
        const [todayRes, weekRes, monthRes, yearRes] = await Promise.all([
          axios.get('/api/reports/sales?period=today', { headers }),
          axios.get('/api/reports/sales?period=week', { headers }),
          axios.get('/api/reports/sales?period=month', { headers }),
          axios.get('/api/reports/sales?period=year', { headers }),
        ]);
        this.salesToday = todayRes.data.data;
        this.salesWeek = weekRes.data.data;
        this.salesMonth = monthRes.data.data;
        this.salesYear = yearRes.data.data;
      } catch (error) {
        console.error('Error fetching sales reports:', error);
      }
    },
    async fetchAdminDashboardReports(params) {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        this.isLoading = true;
        try {
            const response = await axios.get('/api/reports/summary', {
                headers: {
                    Authorization: `Bearer ${authStore.token}`,
                },
                params: {
                    start_date: params.startDate,
                    end_date: params.endDate,
                    store_id: params.storeId,
                },
            });
            this.summaryData = response.data.data;
        } catch (error) {
            console.error('Error fetching report summary:', error);
            this.summaryData = null; // Reset on error
        } finally {
            this.isLoading = false;
        }
    },
  },
});
