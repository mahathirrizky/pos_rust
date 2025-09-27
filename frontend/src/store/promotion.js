import { defineStore } from 'pinia';
import axios from 'axios';

export const usePromotionStore = defineStore('promotion', {
  state: () => ({
    promotions: [],
    isLoading: false,
    error: null,
  }),
  actions: {
    async fetchPromotions() {
      this.isLoading = true;
      this.error = null;
      try {
        const response = await axios.get('/api/admin/promotions');
        this.promotions = response.data.data;
      } catch (error) {
        this.error = error;
        console.error('Error fetching promotions:', error);
      } finally {
        this.isLoading = false;
      }
    },

    async savePromotion(promotion) {
      this.isLoading = true;
      this.error = null;
      try {
        let response;
        if (promotion.id) {
          // Update existing promotion
          response = await axios.put(`/api/admin/promotions/${promotion.id}`, promotion);
          const index = this.promotions.findIndex(p => p.id === promotion.id);
          if (index !== -1) {
            this.promotions[index] = response.data;
          }
        } else {
          // Create new promotion
          response = await axios.post('/api/admin/promotions', promotion);
          this.promotions.push(response.data);
        }
        return response.data;
      } catch (error) {
        this.error = error;
        console.error('Error saving promotion:', error);
        throw error; // Re-throw to allow component to handle
      } finally {
        this.isLoading = false;
      }
    },

    async deletePromotion(promotionId) {
      this.isLoading = true;
      this.error = null;
      try {
        await axios.delete(`/api/admin/promotions/${promotionId}`);
        this.promotions = this.promotions.filter(p => p.id !== promotionId);
      } catch (error) {
        this.error = error;
        console.error('Error deleting promotion:', error);
        throw error; // Re-throw to allow component to handle
      } finally {
        this.isLoading = false;
      }
    },
  },
});
