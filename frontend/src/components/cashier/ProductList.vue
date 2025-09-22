<script setup>
import { ref, onMounted, computed } from 'vue';
// Removed DataView import - ensuring it's fully removed
import Button from 'primevue/button';
import ProgressSpinner from 'primevue/progressspinner';

import InputText from 'primevue/inputtext';
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import Card from 'primevue/card';
// Removed useToast import

const products = ref([]);
const loading = ref(true);
const searchQuery = ref('');
const searchInput = ref(null); // New ref for the input element

// Removed toast instance

// Define the event that this component can emit
const emit = defineEmits(['add-to-cart']);

const filteredProducts = computed(() => {
  if (!searchQuery.value) {
    return products.value;
  }
  return products.value.filter(product =>
    product.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    product.sku.toLowerCase().includes(searchQuery.value.toLowerCase()) // Added SKU search
  );
});

// Replaced fetchProducts with dummy data
const loadDummyProducts = () => {
  loading.value = true;
  products.value = [
    { id: 1, name: 'Laptop', description: 'Powerful laptop for work and gaming', price: 1200.00, sku: 'LAP001', stock: 15 },
    { id: 2, name: 'Mouse', description: 'Ergonomic wireless mouse', price: 25.50, sku: 'MOU002', stock: 50 },
    { id: 3, name: 'Keyboard', description: 'Mechanical gaming keyboard', price: 75.00, sku: 'KEY003', stock: 20 },
    { id: 4, name: 'Monitor', description: '27-inch 4K IPS monitor', price: 350.00, sku: 'MON004', stock: 10 },
    { id: 5, name: 'Webcam', description: 'Full HD webcam with mic', price: 40.00, sku: 'WEB005', stock: 30 },
    { id: 6, name: 'Headphones', description: 'Noise-cancelling over-ear headphones', price: 150.00, sku: 'HEA006', stock: 25 },
    { id: 7, name: 'USB Drive', description: '128GB USB 3.0 flash drive', price: 15.00, sku: 'USB007', stock: 100 },
    { id: 8, name: 'External SSD', description: '1TB portable SSD', price: 99.99, sku: 'SSD008', stock: 8 },
    { id: 9, name: 'Router', description: 'Wi-Fi 6 mesh router', price: 120.00, sku: 'ROU009', stock: 12 },
    { id: 10, name: 'Printer', description: 'All-in-one inkjet printer', price: 80.00, sku: 'PRI010', stock: 5 },
  ];
  loading.value = false;
};

onMounted(() => {
  loadDummyProducts(); // Call dummy data loader
  if (searchInput.value && searchInput.value.$el) {
    searchInput.value.$el.focus(); // Focus the input field
  }
});

const handleAddToCart = (product) => {
  emit('add-to-cart', product);
};

</script>

<template>
  <div class="card h-full flex flex-col">
    <div class="p-4">
      <IconField iconPosition="left" class="w-full">
        <InputIcon class="pi pi-search" />
        <InputText ref="searchInput" v-model="searchQuery" class="w-full block border-2 border-surface-200 rounded-lg" />
      </IconField>
    </div>

    <div class="flex-grow overflow-auto">
      <div v-if="loading" class="flex justify-center items-center h-full">
        <ProgressSpinner />
      </div>
      <div v-else-if="products.length > 0">
        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 p-2">
          <div v-for="product in products" :key="product.id" class="p-2">
            <Card class="h-full flex flex-col">
              <template #header>
                <!-- Placeholder for image -->
                <div class="bg-surface-100 flex items-center justify-center h-40 rounded-t-lg">
                  <i class="pi pi-image text-6xl text-surface-400"></i>
                </div>
              </template>
              <template #content>
                <div class="flex flex-col flex-grow">
                  <h5 class="text-lg font-bold">{{ product.name }}</h5>
                  <p class="text-md line-clamp-1">{{ product.description }}</p>
                  <p class="text-sm text-surface-500 mt-1">SKU: {{ product.sku }}</p>
                  <div class="flex flex-col mt-auto">
                    <span class="text-2xl font-semibold text-primary">Rp{{ product.price }}</span>
                    <span v-if="product.stock !== undefined" :class="{'text-red-500': product.stock < 10, 'text-green-500': product.stock >= 10}" class="text-sm mt-1">
                      Stock: {{ product.stock }}
                    </span>
                  </div>
                </div>
              </template>
              <template #footer>
                <Button label="Add" icon="pi pi-plus" class="mt-4 w-full" @click="handleAddToCart(product)" />
              </template>
            </Card>
          </div>
        </div>
      </div>
      <template v-else>
        <div class="p-4 text-center text-gray-500">
          No products found.
        </div>
      </template>
    </div>
  </div>
</template>












