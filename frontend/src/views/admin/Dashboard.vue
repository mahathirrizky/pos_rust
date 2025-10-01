<script setup>
import { ref, onMounted, computed } from 'vue';
import Card from 'primevue/card';

import Chart from 'primevue/chart';
import { useToast } from "primevue/usetoast";

// Import primary stores, following the Owner dashboard pattern
import { useStoreStore } from '../../store/store';
import { useEmployeeStore } from '../../store/employee';
import { useProductStore } from '../../store/product';
import { useReportStore } from '../../store/report';

const toast = useToast();

// Instantiate the stores
const storeStore = useStoreStore();
const employeeStore = useEmployeeStore();
const productStore = useProductStore();
const reportStore = useReportStore();

const isLoading = ref(true);

const quickLinks = ref([
    { label: 'Manage Stores', icon: 'pi pi-building', to: '/admin/stores' },
    { label: 'Manage Employees', icon: 'pi pi-users', to: '/admin/employees' },
    { label: 'View Reports', icon: 'pi pi-chart-bar', to: '/admin/reports' },
    { label: 'Manage Products', icon: 'pi pi-tag', to: '/admin/products' },
]);

// Chart data and options, adapted from Owner dashboard
const salesChartData = computed(() => {
    return {
      labels: ['Today', 'This Week', 'This Month', 'This Year'],
      datasets: [
        {
          label: 'Sales',
          borderColor: '#42A5F5',
          tension: 0.4,
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

const chartOptions = ref({
  scales: {
    y: {
      beginAtZero: true
    }
  }
});

const formatCurrency = (value) => {
    return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR' }).format(value);
};

onMounted(async () => {
  isLoading.value = true;
  try {
    // Fetch data from primary stores
    await Promise.all([
        storeStore.fetchStores(),
        employeeStore.fetchEmployees({ roles_to_exclude: 'Owner' }),
        productStore.fetchProducts(),
        reportStore.fetchSalesReports(),
    ]);
  } catch (error) {
    toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to load dashboard data', life: 3000 });
  } finally {
    isLoading.value = false;
  }
});

</script>

<template>
  <div class="p-4">
    <h1 class="text-3xl font-bold mb-6">Admin Dashboard</h1>

    <div v-if="isLoading" class="text-center py-8">
      <i class="pi pi-spin pi-spinner text-4xl text-blue-500"></i>
      <p class="text-lg mt-2">Loading Dashboard Data...</p>
    </div>
    <!-- Error handling can be added here if desired -->
    <div v-else>
      <!-- Summary Cards -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-6">
        <Card class="hover:shadow-lg transition-shadow duration-300">
          <template #title>
            <div class="flex items-center justify-between">
              <span class="text-lg font-semibold">Total Stores</span>
              <i class="pi pi-building text-2xl text-blue-500"></i>
            </div>
          </template>
          <template #content><div class="text-4xl font-bold text-surface-800 dark:text-surface-100">{{ storeStore.stores.length }}</div></template>
        </Card>
        <Card class="hover:shadow-lg transition-shadow duration-300">
          <template #title>
            <div class="flex items-center justify-between">
              <span class="text-lg font-semibold">Total Employees</span>
              <i class="pi pi-users text-2xl text-green-500"></i>
            </div>
          </template>
          <template #content><div class="text-4xl font-bold text-surface-800 dark:text-surface-100">{{ employeeStore.employees.length }}</div></template>
        </Card>
        <Card class="hover:shadow-lg transition-shadow duration-300">
          <template #title>
            <div class="flex items-center justify-between">
              <span class="text-lg font-semibold">Total Products</span>
              <i class="pi pi-tag text-2xl text-purple-500"></i>
            </div>
          </template>
          <template #content><div class="text-4xl font-bold text-surface-800 dark:text-surface-100">{{ productStore.products.length }}</div></template>
        </Card>
        <Card class="hover:shadow-lg transition-shadow duration-300">
          <template #title>
            <div class="flex items-center justify-between">
              <span class="text-lg font-semibold">Sales Today</span>
              <i class="pi pi-dollar text-2xl text-yellow-500"></i>
            </div>
          </template>
          <template #content><div class="text-4xl font-bold text-surface-800 dark:text-surface-100">{{ formatCurrency(reportStore.salesToday || 0) }}</div></template>
        </Card>
      </div>

      <!-- Sales Chart & Quick Actions -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6">
        <div class="lg:col-span-2">
          <Card class="h-full">
            <template #title><div class="text-xl font-semibold">Sales Overview</div></template>
            <template #content>
                <Chart type="line" :data="salesChartData" :options="chartOptions" style="height: 300px" />
            </template>
          </Card>
        </div>
        <div>
          <Card class="h-full">
            <template #title><div class="text-xl font-semibold">Quick Actions</div></template>
            <template #content>
              <div class="grid grid-cols-2 gap-4">
                <router-link v-for="link in quickLinks" :key="link.to" :to="link.to" class="block p-4 text-center bg-surface-100 dark:bg-surface-800 hover:bg-blue-100 rounded-lg transition-colors duration-300">
                  <i :class="[link.icon, 'text-4xl text-blue-500 mb-4']"></i>
                  <span class="font-semibold text-surface-800 dark:text-surface-100">{{ link.label }}</span>
                </router-link>
              </div>
            </template>
          </Card>
        </div>
      </div>

      <!-- Recent Activities -->
       <Card>
        <template #title><div class="text-xl font-semibold">Recent Activities</div></template>
        <template #content>
          <p class="text-surface-500 dark:text-surface-400">This feature is not yet available.</p>
        </template>
      </Card>
    </div>
  </div>
</template>