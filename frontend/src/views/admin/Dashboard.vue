<script setup>
import { ref, onMounted, computed } from 'vue';
import Card from 'primevue/card';
import Button from 'primevue/button';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
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

const chartOptions = ref({
  scales: {
    y: {
      beginAtZero: true
    }
  }
});

const formatCurrency = (value) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(value);
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
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        <Card>
          <template #title><div class="text-lg">Total Stores</div></template>
          <template #content><div class="text-4xl font-bold">{{ storeStore.stores.length }}</div></template>
        </Card>
        <Card>
          <template #title><div class="text-lg">Total Employees</div></template>
          <template #content><div class="text-4xl font-bold">{{ employeeStore.employees.length }}</div></template>
        </Card>
        <Card>
          <template #title><div class="text-lg">Total Products</div></template>
          <template #content><div class="text-4xl font-bold">{{ productStore.products.length }}</div></template>
        </Card>
        <Card>
          <template #title><div class="text-lg">Sales Today</div></template>
          <template #content><div class="text-4xl font-bold">{{ formatCurrency(reportStore.salesToday || 0) }}</div></template>
        </Card>
      </div>

      <!-- Sales Chart -->
      <Card class="mb-6">
          <template #title><div class="text-xl font-semibold">Sales Overview</div></template>
          <template #content>
              <Chart type="bar" :data="salesChartData" :options="chartOptions" style="height: 300px" />
          </template>
      </Card>

      <!-- Quick Links -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card>
          <template #title><div class="text-xl font-semibold">Quick Links</div></template>
          <template #content>
            <div class="grid grid-cols-2 gap-4">
              <router-link v-for="link in quickLinks" :key="link.to" :to="link.to" class="block">
                <Button :label="link.label" :icon="link.icon" class="w-full" severity="secondary" outlined />
              </router-link>
            </div>
          </template>
        </Card>
        <!-- The "Recent Activities" card is removed for now as there is no data source for it -->
         <Card>
          <template #title><div class="text-xl font-semibold">Recent Activities</div></template>
          <template #content>
            <p class="text-gray-500">This feature is not yet available.</p>
          </template>
        </Card>
      </div>
    </div>
  </div>
</template>