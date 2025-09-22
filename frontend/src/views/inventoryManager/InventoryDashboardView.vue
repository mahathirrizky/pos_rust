<template>
  <div class="p-4">
    <h1 class="text-2xl font-bold mb-4">Inventory Dashboard</h1>
    <p>Welcome to the Inventory Manager Dashboard. Here you can manage inventory, products, and suppliers.</p>

    <div v-if="inventoryItems.length > 0" class="mt-6">
      <h2 class="text-xl font-semibold mb-3">Current Inventory</h2>
      <table class="min-w-full bg-white border border-gray-200">
        <thead>
          <tr>
            <th class="py-2 px-4 border-b">Product Name</th>
            <th class="py-2 px-4 border-b">Store Name</th>
            <th class="py-2 px-4 border-b">Quantity</th>
            <th class="py-2 px-4 border-b">Last Restocked</th>
            <th class="py-2 px-4 border-b">Updated At</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in inventoryItems" :key="item.inventory_id">
            <td class="py-2 px-4 border-b">{{ item.product_name }}</td>
            <td class="py-2 px-4 border-b">{{ item.store_name }}</td>
            <td class="py-2 px-4 border-b">{{ item.quantity }}</td>
            <td class="py-2 px-4 border-b">{{ item.last_restocked ? new Date(item.last_restocked).toLocaleDateString() : 'N/A' }}</td>
            <td class="py-2 px-4 border-b">{{ new Date(item.updated_at).toLocaleDateString() }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-else class="mt-6 text-gray-600">
      Loading inventory data or no items found...
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';

const inventoryItems = ref([]);

const fetchInventory = async () => {
  try {
    const response = await fetch('/api/inventory/report', {
      headers: {
        // Assuming a token is needed for authorization
        // Replace with actual token retrieval logic
        'Authorization': 'Bearer YOUR_AUTH_TOKEN'
      }
    });
    const data = await response.json();
    if (response.ok) {
      inventoryItems.value = data.data.items;
    } else {
      console.error('Failed to fetch inventory:', data.message);
      // Handle error, e.g., show a toast notification
    }
  } catch (error) {
    console.error('Error fetching inventory:', error);
    // Handle network error
  }
};

onMounted(() => {
  fetchInventory();
});
</script>

<style scoped>
/* Scoped styles for this component */
</style>