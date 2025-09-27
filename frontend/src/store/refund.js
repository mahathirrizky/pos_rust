import { defineStore } from 'pinia';
import axios from 'axios';

export const useRefundStore = defineStore('refund', {
  state: () => ({
    refunds: [],
    isLoading: false,
    error: null,
  }),
  actions: {
    async fetchRefunds() {
      this.isLoading = true;
      this.error = null;
      try {
        const response = await axios.get('/api/refunds');
        this.refunds = response.data.data;
      } catch (error) {
        this.error = error;
        console.error('Error fetching refunds:', error);
      } finally {
        this.isLoading = false;
      }
    },
  },
});
