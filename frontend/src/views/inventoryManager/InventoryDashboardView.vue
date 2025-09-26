<script setup>
import { ref, onMounted, computed } from 'vue';
import { useToast } from "primevue/usetoast";
import Card from 'primevue/card';
import Button from 'primevue/button';
import ProgressSpinner from 'primevue/progressspinner';
import Chart from 'primevue/chart';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import { useDashboardStore } from '../../store/dashboard';

const toast = useToast();
const dashboardStore = useDashboardStore();

const isLoading = ref(true);

const categoryChartData = computed(() => {
    if (!dashboardStore.products || !dashboardStore.categories) {
        return null;
    }
    const categoryCounts = {};
    dashboardStore.products.forEach(product => {
      const category = dashboardStore.categories.find(cat => cat.id === product.category_id);
      if (category) {
        categoryCounts[category.name] = (categoryCounts[category.name] || 0) + 1;
      }
    });

    return {
      labels: Object.keys(categoryCounts),
      datasets: [
        {
          data: Object.values(categoryCounts),
          backgroundColor: [
            "#42A5F5", "#66BB6A", "#FFA726", "#26C6DA", "#7E57C2", "#FF7043", "#8D6E63", "#BDBDBD",
          ],
          hoverBackgroundColor: [
            "#64B5F6", "#81C784", "#FFB74D", "#4DD0E1", "#9575CD", "#FF8A65", "#A1887F", "#E0E0E0",
          ],
        },
      ],
    };
});

const categoryChartOptions = ref({
  plugins: {
    legend: {
      labels: {
        usePointStyle: true,
      },
    },
  },
});

const lowStockProducts = computed(() => {
    if (!dashboardStore.inventoryReport) {
        return [];
    }
    return dashboardStore.inventoryReport.items.filter(item => item.quantity < 10);
});

onMounted(async () => {
  isLoading.value = true;
  try {
    await dashboardStore.fetchDashboardData();
  } catch (err) {
    toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 5000 });
  } finally {
    isLoading.value = false;
  }
});

const formatDate = (value) => {
  if (!value) return '';
  return new Date(value).toLocaleString();
};
</script>

<template>
  <div class="p-4">
    <h1 class="text-2xl font-bold mb-4">Inventory Dashboard</h1>

    <div v-if="isLoading" class="text-center py-8">
      <ProgressSpinner />
      <p class="text-lg mt-2">Loading Inventory Dashboard...</p>
    </div>

    <div v-else-if="!dashboardStore.products" class="text-center py-8 text-red-500">
      <p>Failed to load dashboard data.</p>
      <Button label="Retry" @click="dashboardStore.fetchDashboardData" class="mt-4" />
    </div>

    <div v-else>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <Card>
          <template #title>Total Products</template>
          <template #content>
            <div class="text-5xl font-bold text-primary">{{ dashboardStore.products.length }}</div>
          </template>
        </Card>
        <Card>
          <template #title>Total Suppliers</template>
          <template #content>
            <div class="text-5xl font-bold text-green-500">{{ dashboardStore.suppliers.length }}</div>
          </template>
        </Card>
        <Card>
          <template #title>Total Categories</template>
          <template #content>
            <div class="text-5xl font-bold text-orange-500">{{ dashboardStore.categories.length }}</div>
          </template>
        </Card>
        <Card>
          <template #title>Low Stock Items</template>
          <template #content>
            <div class="text-5xl font-bold" :class="{'text-red-500': lowStockProducts.length > 0, 'text-green-500': lowStockProducts.length === 0}">
              {{ lowStockProducts.length }}
            </div>
          </template>
        </Card>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <Card v-if="lowStockProducts.length > 0">
          <template #title>Low Stock Alerts</template>
          <template #content>
            <ul class="list-disc pl-5">
              <li v-for="item in lowStockProducts" :key="item.inventory_id" class="mb-1">
                {{ item.product_name }} (Store: {{ item.store_name }}, Qty: {{ item.quantity }})
              </li>
            </ul>
          </template>
        </Card>

        <Card v-if="categoryChartData && categoryChartData.labels.length > 0">
          <template #title>Products by Category</template>
          <template #content>
            <Chart type="pie" :data="categoryChartData" :options="categoryChartOptions" class="w-full md:w-[30rem]" />
          </template>
        </Card>
      </div>

      <div class="mt-8">
        <h2 class="text-xl font-semibold mb-3">Full Inventory Overview</h2>
        <DataTable :value="dashboardStore.inventoryReport.items" responsiveLayout="scroll">
          <Column field="product_name" header="Product" :sortable="true"></Column>
          <Column field="store_name" header="Store" :sortable="true"></Column>
          <Column field="quantity" header="Quantity" :sortable="true"></Column>
          <Column field="last_restocked" header="Last Restocked" :sortable="true">
            <template #body="slotProps">
              {{ formatDate(slotProps.data.last_restocked) }}
            </template>
          </Column>
          <Column field="updated_at" header="Last Updated" :sortable="true">
            <template #body="slotProps">
              {{ formatDate(slotProps.data.updated_at) }}
            </template>
          </Column>
        </DataTable>
      </div>
    </div>
  </div>
</template>