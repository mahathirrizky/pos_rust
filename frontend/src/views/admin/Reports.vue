<script setup>
import { ref, onMounted } from 'vue';
import Card from 'primevue/card';
import Chart from 'primevue/chart';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Toolbar from 'primevue/toolbar';
import Calendar from 'primevue/calendar';
import Dropdown from 'primevue/dropdown';
import Button from 'primevue/button';

const dateRange = ref();
const selectedStore = ref();
const stores = ref(['All Stores', 'Main Street Store', 'Downtown Branch', 'Uptown Plaza']);

const summaryData = ref({});
const topProducts = ref([]);
const topEmployees = ref([]);
const salesChartData = ref({});
const chartOptions = ref({});

// Dummy data mimicking a backend report response
const dummyReportData = {
    summary: {
        total_revenue: 152340.75,
        total_orders: 890,
        average_order_value: 171.17
    },
    top_products: [
        { product_name: 'Laptop', quantity_sold: 40, total_revenue: 48020.00 },
        { product_name: 'Keyboard', quantity_sold: 150, total_revenue: 22612.50 },
        { product_name: 'Gaming PC', quantity_sold: 8, total_revenue: 19200.00 },
        { product_name: 'Mouse', quantity_sold: 200, total_revenue: 15000.00 },
    ],
    top_employees: [
        { employee_name: 'Jane Smith', total_sales_handled: 55430.20 },
        { employee_name: 'John Doe', total_sales_handled: 48760.50 },
        { employee_name: 'Peter Jones', total_sales_handled: 35100.05 },
    ],
    sales_over_time: {
        labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September'],
        datasets: [
            {
                label: 'Sales',
                data: [12000, 19000, 15000, 21000, 18000, 23000, 25000, 22000, 28000],
                borderColor: '#4F46E5',
                tension: 0.4
            }
        ]
    }
};

onMounted(() => {
  summaryData.value = dummyReportData.summary;
  topProducts.value = dummyReportData.top_products;
  topEmployees.value = dummyReportData.top_employees;
  salesChartData.value = dummyReportData.sales_over_time;
  chartOptions.value = getChartOptions();
});

const formatCurrency = (value) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(value);
};

const getChartOptions = () => {
    const documentStyle = getComputedStyle(document.documentElement);
    const textColor = documentStyle.getPropertyValue('--p-text-color');
    const textColorSecondary = documentStyle.getPropertyValue('--p-text-muted-color');
    const surfaceBorder = documentStyle.getPropertyValue('--p-content-border-color');

    return {
        maintainAspectRatio: false,
        aspectRatio: 0.6,
        plugins: { legend: { labels: { color: textColor } } },
        scales: {
            x: { ticks: { color: textColorSecondary }, grid: { color: surfaceBorder } },
            y: { ticks: { color: textColorSecondary }, grid: { color: surfaceBorder } }
        }
    };
};

const applyFilters = () => {
    // In a real app, this would fetch new data from the backend
    console.log("Applying filters:", { dateRange: dateRange.value, store: selectedStore.value });
    alert('Filtering would be applied here.');
};

</script>

<template>
  <div>
    <Toolbar class="mb-4">
        <template #start>
            <div class="flex items-center gap-2">
                <Calendar v-model="dateRange" selectionMode="range" :manualInput="false" placeholder="Select Date Range" class="w-full md:w-20rem" />
                <Dropdown v-model="selectedStore" :options="stores" placeholder="Select a Store" class="w-full md:w-14rem" />
                <Button label="Apply" icon="pi pi-check" @click="applyFilters" />
            </div>
        </template>
    </Toolbar>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-4 mb-4">
        <Card>
            <template #title><div class="text-lg">Total Revenue</div></template>
            <template #content><div class="text-3xl font-bold">{{ formatCurrency(summaryData.total_revenue) }}</div></template>
        </Card>
        <Card>
            <template #title><div class="text-lg">Total Orders</div></template>
            <template #content><div class="text-3xl font-bold">{{ summaryData.total_orders }}</div></template>
        </Card>
        <Card>
            <template #title><div class="text-lg">Average Order Value</div></template>
            <template #content><div class="text-3xl font-bold">{{ formatCurrency(summaryData.average_order_value) }}</div></template>
        </Card>
    </div>

    <Card class="mb-4">
        <template #title><div class="text-xl font-semibold">Sales Over Time</div></template>
        <template #content>
            <Chart type="line" :data="salesChartData" :options="chartOptions" style="height: 300px" />
        </template>
    </Card>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <Card>
            <template #title><div class="text-xl font-semibold">Top Selling Products</div></template>
            <template #content>
                <DataTable :value="topProducts" responsiveLayout="scroll">
                    <Column field="product_name" header="Product"></Column>
                    <Column field="quantity_sold" header="Quantity Sold"></Column>
                    <Column field="total_revenue" header="Total Revenue">
                        <template #body="slotProps">{{ formatCurrency(slotProps.data.total_revenue) }}</template>
                    </Column>
                </DataTable>
            </template>
        </Card>
        <Card>
            <template #title><div class="text-xl font-semibold">Top Performing Employees</div></template>
            <template #content>
                <DataTable :value="topEmployees" responsiveLayout="scroll">
                    <Column field="employee_name" header="Employee"></Column>
                    <Column field="total_sales_handled" header="Total Sales">
                        <template #body="slotProps">{{ formatCurrency(slotProps.data.total_sales_handled) }}</template>
                    </Column>
                </DataTable>
            </template>
        </Card>
    </div>

  </div>
</template>
