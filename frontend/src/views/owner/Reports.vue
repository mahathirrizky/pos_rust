<script setup>
import { ref, onMounted, computed, watch, onBeforeUnmount } from 'vue';
import Card from 'primevue/card';
import Button from 'primevue/button';
import Chart from 'primevue/chart';
import DatePicker from 'primevue/datepicker';
import Select from 'primevue/select';
import { useOwnerReportStore } from '../../store/owner_reports';
import { useStoreStore } from '../../store/store';

const ownerReportStore = useOwnerReportStore();
const storeStore = useStoreStore();

// Create a default date range to prevent double-fetching on mount
const createDefaultDateRange = () => {
  const endDate = new Date();
  const startDate = new Date();
  startDate.setDate(endDate.getDate() - 7);
  return [startDate, endDate];
};

const dateRange = ref(createDefaultDateRange());
const selectedStore = ref(null);
const salesChart = ref();
const salesByStoreChart = ref();
const isMounted = ref(false); // Control chart rendering

onMounted(() => {
  storeStore.fetchStores();
  // Initial fetch
  fetchReports();
  isMounted.value = true; // Allow charts to render
});

onBeforeUnmount(() => {
  isMounted.value = false; // Immediately remove charts from DOM
  if (salesChart.value) {
    const chartInstance = salesChart.value.getChart();
    if (chartInstance) {
      chartInstance.destroy();
    }
  }
  if (salesByStoreChart.value) {
    const chartInstance = salesByStoreChart.value.getChart();
    if (chartInstance) {
      chartInstance.destroy();
    }
  }
});

const fetchReports = () => {
  if (!dateRange.value || dateRange.value.length !== 2 || !dateRange.value[0] || !dateRange.value[1]) {
    return;
  }
  const params = {
    startDate: dateRange.value[0].toISOString().split('T')[0],
    endDate: dateRange.value[1].toISOString().split('T')[0],
    storeId: selectedStore.value,
  };
  ownerReportStore.fetchSummary(params);
};

// Watch for filter changes
watch([dateRange, selectedStore], fetchReports, { deep: true });

const formatCurrency = (value) => {
  if (typeof value !== 'number') return '';
  return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(value);
};

const salesChartData = computed(() => {
  const data = ownerReportStore.summaryData?.sales_over_time;
  if (!data) {
    return {};
  }
  return {
    labels: data.map(item => new Date(item[0] + 'T00:00:00').toLocaleDateString('en-US', { month: 'short', day: 'numeric' })), // Ensure date is parsed correctly
    datasets: [
      {
        label: 'Daily Sales',
        data: data.map(item => item[1]),
        fill: true,
        borderColor: '#42A5F5',
        tension: 0.4,
        backgroundColor: 'rgba(66, 165, 245, 0.2)',
      },
    ],
  };
});

const salesByStoreChartData = computed(() => {
  const data = ownerReportStore.summaryData?.sales_by_store;
  if (!data || data.length === 0) {
    return {};
  }
  return {
    labels: data.map(item => item[0]), // Store names
    datasets: [
      {
        label: 'Sales by Store',
        data: data.map(item => item[1]), // Sales figures
        backgroundColor: [
          '#42A5F5',
          '#66BB6A',
          '#FFA726',
          '#26A69A',
          '#AB47BC',
          '#EC407A',
          '#78909C',
        ],
      },
    ],
  };
});

const chartOptions = ref({
    maintainAspectRatio: false,
    aspectRatio: 0.6,
    plugins: {
        legend: {
            labels: {
                color: '#495057'
            }
        }
    },
    scales: {
        x: {
            ticks: {
                color: '#495057'
            },
            grid: {
                color: '#ebedef'
            }
        },
        y: {
            ticks: {
                color: '#495057'
            },
            grid: {
                color: '#ebedef'
            }
        }
    }
});

const barChartOptions = ref({
    plugins: {
        legend: {
            display: false // Hide legend for bar chart with single dataset
        }
    },
    scales: {
        y: {
            beginAtZero: true,
            ticks: {
                color: '#495057'
            },
            grid: {
                color: '#ebedef'
            }
        },
        x: {
            ticks: {
                color: '#495057'
            },
            grid: {
                color: '#ebedef'
            }
        }
    }
});

</script>

<template>
  <div>
    <Card>
      <template #title>
        <span class="text-2xl font-semibold">Business Reports</span>
      </template>
      <template #content>
        <!-- Filters -->
        <div class="flex flex-wrap gap-4 mb-6 p-4 bg-gray-50 rounded-lg">
          <div class="flex-auto">
            <label for="dr" class="font-bold block mb-2"> Date Range </label>
            <DatePicker v-model="dateRange" selectionMode="range" :manualInput="false" showIcon inputId="dr" class="w-full"/>
          </div>
          <div class="flex-auto">
            <label for="store" class="font-bold block mb-2"> Store </label>
            <Select v-model="selectedStore" :options="storeStore.stores" optionLabel="name" optionValue="id" placeholder="All Stores" showClear inputId="store" class="w-full"/>
          </div>
        </div>

        <!-- Loading Spinner -->
        <div v-if="ownerReportStore.isLoading" class="text-center py-8">
          <i class="pi pi-spin pi-spinner text-4xl text-blue-500"></i>
          <p class="text-lg mt-2">Loading Reports...</p>
        </div>

        <!-- Report Content -->
        <div v-else-if="ownerReportStore.summaryData">
          <!-- Key Metrics -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
            <Card>
              <template #title>Total Revenue</template>
              <template #content>
                <div class="text-4xl font-bold text-green-500">
                  {{ formatCurrency(ownerReportStore.summaryData.total_revenue) }}
                </div>
              </template>
            </Card>
            <Card>
              <template #title>Total Transactions</template>
              <template #content>
                <div class="text-4xl font-bold text-blue-500">
                  {{ ownerReportStore.summaryData.total_transactions }}
                </div>
              </template>
            </Card>
          </div>

          <!-- Charts -->
          <div v-if="isMounted" class="grid grid-cols-1 lg:grid-cols-2 gap-4">
            <Card>
              <template #title>Sales Over Time</template>
              <template #content>
                <Chart ref="salesChart" type="line" :data="salesChartData" :options="chartOptions" style="height: 300px" />
              </template>
            </Card>
            <Card>
              <template #title>Sales by Store</template>
              <template #content>
                <Chart ref="salesByStoreChart" type="bar" :data="salesByStoreChartData" :options="barChartOptions" style="height: 300px" />
              </template>
            </Card>
          </div>
        </div>
        
        <!-- No Data Message -->
        <div v-else class="text-center py-8 bg-gray-50 rounded-lg">
            <i class="pi pi-info-circle text-4xl text-gray-400"></i>
            <p class="text-lg mt-2 text-gray-600">No data available for the selected filters.</p>
        </div>

      </template>
    </Card>
  </div>
</template>

