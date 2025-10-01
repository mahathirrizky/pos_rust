<script setup>
import { ref, onMounted, computed } from 'vue';
import Card from 'primevue/card';
import Button from 'primevue/button';
import Chart from 'primevue/chart';
import { useStoreStore } from '../../store/store';
import { useEmployeeStore } from '../../store/employee';
import { useProductStore } from '../../store/product';
import { useReportStore } from '../../store/report';

const storeStore = useStoreStore();
const employeeStore = useEmployeeStore();
const productStore = useProductStore();
const reportStore = useReportStore();

const isLoading = ref(true);

const salesChartData = computed(() => {
    return {
      labels: ['Today', 'This Week', 'This Month', 'This Year'],
      datasets: [
        {
          label: 'Sales',
          backgroundColor: '#42A5F5',
          data: [
            reportStore.salesToday || 0,
            reportStore.salesWeek || 0,
            reportStore.salesMonth || 0,
            reportStore.salesYear || 0
          ]
        }
      ]
    };
});

const salesChartOptions = ref({
  maintainAspectRatio: false,
  aspectRatio: 0.6,
  scales: {
    y: {
      beginAtZero: true
    }
  }
});

onMounted(async () => {
  isLoading.value = true;
  try {
    await Promise.all([
        storeStore.fetchStores(),
        employeeStore.fetchEmployees({ roles_to_exclude: 'Owner' }),
        productStore.fetchProducts(),
        reportStore.fetchSalesReports(),
    ]);
  } catch (err) {
    console.error(err);
  } finally {
    isLoading.value = false;
  }
});
</script>

<template>
  <div>
    <h1 class="text-2xl font-semibold mb-4">Owner Dashboard</h1>

    <div v-if="isLoading" class="text-center py-8">
      <i class="pi pi-spin pi-spinner text-4xl text-blue-500"></i>
      <p class="text-lg mt-2">Loading Dashboard Data...</p>
    </div>

    <div v-else>
      <div class="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-4">
        <Card>
          <template #title>Total Stores</template>
          <template #content>
            <div class="text-5xl font-bold text-primary">{{ storeStore.stores.length }}</div>
          </template>
        </Card>

        <Card>
          <template #title>Total Employees</template>
          <template #content>
            <div class="text-5xl font-bold text-green-500">{{ employeeStore.employees.length }}</div>
          </template>
        </Card>

        <Card>
          <template #title>Total Products</template>
          <template #content>
            <div class="text-5xl font-bold text-orange-500">{{ productStore.products.length }}</div>
          </template>
        </Card>

        <Card>
          <template #title>Sales Today</template>
          <template #content>
            <div class="text-4xl font-bold text-purple-500">${{ (reportStore.salesToday || 0).toFixed(2) }}</div>
          </template>
        </Card>
      </div>

      <div class="mt-8">
        <Card>
          <template #title>Sales Overview</template>
          <template #content>
            <Chart type="bar" :data="salesChartData" :options="salesChartOptions" />
          </template>
        </Card>
      </div>

      <p class="mt-8 text-surface-600 dark:text-surface-300">More high-level business overview and reports will be displayed here.</p>
    </div>
  </div>
</template>