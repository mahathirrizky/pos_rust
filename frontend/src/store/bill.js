import { defineStore } from 'pinia';
import axios from 'axios';
import { ref } from 'vue';
import { useAuthStore } from './auth'; // Needed for token

export const useBillStore = defineStore('bill', () => {
  // STATE
  const bills = ref([]);
  const isLoading = ref(false);
  const error = ref(null);

  // ACTIONS
  const authStore = useAuthStore();

  async function fetchBills(filters = {}) {
    isLoading.value = true;
    error.value = null;
    try {
      const params = new URLSearchParams();
      if (filters.date) {
        params.append('date', filters.date);
      }
      if (filters.status) {
        params.append('status', filters.status);
      }
      // This seems to be an admin-only endpoint.
      // We should ensure the token is included.
      const response = await axios.get('/api/admin/bills', {
        params,
        headers: { Authorization: `Bearer ${authStore.token}` }
      });
      bills.value = response.data.data; // FIX: Access data.data
    } catch (e) {
      error.value = e;
      console.error('Error fetching bills:', e);
    } finally {
      isLoading.value = false;
    }
  }

  async function submitOrder(orderPayload) {
    if (!authStore.token) {
      throw new Error('Authentication token not found.');
    }
    // The action itself doesn't need a try/catch if we want the component to handle it.
    // It will throw an error automatically on a non-2xx response.
    await axios.post('/api/orders', orderPayload, {
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${authStore.token}`,
      },
    });
    // On success, we could potentially refetch bills if the view is live,
    // but for now, the component handles the post-success logic.
  }

  return {
    bills,
    isLoading,
    error,
    fetchBills,
    submitOrder,
  };
});
