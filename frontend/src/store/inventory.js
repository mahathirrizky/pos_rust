import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth';

export const useInventoryStore = defineStore('inventory', {
  state: () => ({
    inventoryReport: null,
  }),
  actions: {
    async fetchInventoryReport() {
      const authStore = useAuthStore();
      if (!authStore.token) {
        console.error('No token found, user is not authenticated');
        return;
      }
      try {
        const response = await axios.get('/api/inventory/report', {
          headers: {
            Authorization: `Bearer ${authStore.token}`,
          },
        });
        this.inventoryReport = response.data;
      } catch (error) {
        console.error('Error fetching inventory report:', error);
      }
    },
    async updateInventoryItem(item) {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        const url = `/api/inventory/${item.inventory_id}`;
        try {
            await axios.put(url, item, {
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            await this.fetchInventoryReport();
        } catch (error) {
            console.error('Error updating inventory item:', error);
            throw error;
        }
    },
  },
});