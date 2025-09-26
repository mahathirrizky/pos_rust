import { defineStore } from 'pinia';
import axios from 'axios';
import { useAuthStore } from './auth';

export const useStoreStore = defineStore('store', {
  state: () => ({
    stores: [],
  }),
  actions: {
    async fetchStores() {
      const authStore = useAuthStore();
      if (!authStore.token) {
        console.error('No token found, user is not authenticated');
        return;
      }
      try {
        const response = await axios.get('/api/stores', {
          headers: {
            Authorization: `Bearer ${authStore.token}`,
          },
        });
        this.stores = response.data.data;
      } catch (error) {
        console.error('Error fetching stores:', error);
      }
    },
    async saveStore(store) {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        const isUpdate = !!store.id;
        const url = isUpdate ? `/api/stores/${store.id}` : '/api/stores';
        const method = isUpdate ? 'put' : 'post';

        try {
            await axios({
                method,
                url,
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `Bearer ${authStore.token}`,
                },
                data: store,
            });
            await this.fetchStores();
        } catch (error) {
            console.error('Error saving store:', error);
            throw error;
        }
    },
    async deleteStore(storeId) {
        const authStore = useAuthStore();
        if (!authStore.token) {
            console.error('No token found, user is not authenticated');
            return;
        }
        try {
            await axios.delete(`/api/stores/${storeId}`, {
                headers: {
                    Authorization: `Bearer ${authStore.token}`,
                },
            });
            await this.fetchStores();
        } catch (error) {
            console.error('Error deleting store:', error);
            throw error;
        }
    },
  },
});