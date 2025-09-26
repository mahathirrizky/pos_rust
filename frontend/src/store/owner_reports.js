import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth';

export const useOwnerReportStore = defineStore('ownerReport', {
  state: () => ({
    summaryData: null,
    isLoading: false,
  }),
  actions: {
    async fetchSummary(params) {
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
