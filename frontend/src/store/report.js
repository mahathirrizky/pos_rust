import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth';

export const useReportStore = defineStore('report', {
  state: () => ({
    salesToday: null,
    salesWeek: null,
    salesMonth: null,
    salesYear: null,
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
  },
});