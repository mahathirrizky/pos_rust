<script setup>
import { ref, onMounted } from 'vue';
import Card from 'primevue/card';
import Button from 'primevue/button';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Chart from 'primevue/chart'; // Import Chart component

const summaryData = ref({
    totalStores: 3,
    totalEmployees: 25,
    totalProducts: 150,
    totalSalesToday: 5230.75,
});

const recentActivities = ref([
    { id: 1, type: 'Order', description: 'New order #1005 placed by Alice J.', date: '2025-09-23T10:30:00Z' },
    { id: 2, type: 'Inventory', description: 'Laptop stock updated in Main Street Store.', date: '2025-09-23T09:45:00Z' },
    { id: 3, type: 'Employee', description: 'New employee John D. added.', date: '2025-09-22T16:00:00Z' },
    { id: 4, type: 'Promotion', description: '10% Off promotion created.', date: '2025-09-22T14:15:00Z' },
]);

const quickLinks = ref([
    { label: 'Manage Stores', icon: 'pi pi-building', to: '/admin/stores' },
    { label: 'Manage Employees', icon: 'pi pi-users', to: '/admin/employees' },
    { label: 'View Reports', icon: 'pi pi-chart-bar', to: '/admin/reports' },
    { label: 'Manage Products', icon: 'pi pi-tag', to: '/admin/products' },
]);

// Chart data and options
const salesChartData = ref({});
const chartOptions = ref({});

const formatCurrency = (value) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(value);
};

const formatDate = (value) => {
  return new Date(value).toLocaleString('en-US', { year: 'numeric', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
};

const getChartOptions = () => {
    const documentStyle = getComputedStyle(document.documentElement);
    const textColor = documentStyle.getPropertyValue('--p-text-color');
    const textColorSecondary = documentStyle.getPropertyValue('--p-text-muted-color');
    const surfaceBorder = documentStyle.getPropertyValue('--p-content-border-color');

    return {
        maintainAspectRatio: false,
        aspectRatio: 0.8,
        plugins: { legend: { labels: { color: textColor } } },
        scales: {
            x: { ticks: { color: textColorSecondary }, grid: { color: surfaceBorder } },
            y: { ticks: { color: textColorSecondary }, grid: { color: surfaceBorder } }
        }
    };
};

onMounted(() => {
  // Dummy data for sales chart
  const documentStyle = getComputedStyle(document.documentElement);
  const textColor = documentStyle.getPropertyValue('--p-text-color');

  salesChartData.value = {
    labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July'],
    datasets: [
        {
            label: 'Sales',
            backgroundColor: documentStyle.getPropertyValue('--p-primary-color'),
            borderColor: documentStyle.getPropertyValue('--p-primary-color'),
            data: [6500, 5900, 8000, 8100, 5600, 5500, 4000]
        }
    ]
  };

  chartOptions.value = getChartOptions();
});

</script>

<template>
  <div class="p-4">
    <h1 class="text-3xl font-bold mb-6">Admin Dashboard</h1>

    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
      <Card>
        <template #title><div class="text-lg">Total Stores</div></template>
        <template #content><div class="text-4xl font-bold">{{ summaryData.totalStores }}</div></template>
      </Card>
      <Card>
        <template #title><div class="text-lg">Total Employees</div></template>
        <template #content><div class="text-4xl font-bold">{{ summaryData.totalEmployees }}</div></template>
      </Card>
      <Card>
        <template #title><div class="text-lg">Total Products</div></template>
        <template #content><div class="text-4xl font-bold">{{ summaryData.totalProducts }}</div></template>
      </Card>
      <Card>
        <template #title><div class="text-lg">Sales Today</div></template>
        <template #content><div class="text-4xl font-bold">{{ formatCurrency(summaryData.totalSalesToday) }}</div></template>
      </Card>
    </div>

    <Card class="mb-6">
        <template #title><div class="text-xl font-semibold">Sales Overview</div></template>
        <template #content>
            <Chart type="bar" :data="salesChartData" :options="chartOptions" style="height: 300px" />
        </template>
    </Card>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <Card>
        <template #title><div class="text-xl font-semibold">Recent Activities</div></template>
        <template #content>
          <DataTable :value="recentActivities" responsiveLayout="scroll">
            <Column field="type" header="Type"></Column>
            <Column field="description" header="Description"></Column>
            <Column field="date" header="Date">
              <template #body="slotProps">{{ formatDate(slotProps.data.date) }}</template>
            </Column>
          </DataTable>
        </template>
      </Card>

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
    </div>
  </div>
</template>