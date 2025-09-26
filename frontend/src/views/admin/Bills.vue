<script setup>
import { ref, onMounted, computed } from 'vue';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import Tag from 'primevue/tag';
import DatePicker from 'primevue/datepicker';
import Select from 'primevue/select';

const bills = ref([]);
const selectedBill = ref(null);
const billDetailsDialog = ref(false);

// Filters
const filterDate = ref();
const filterStatus = ref();
const statuses = ref(['Completed', 'Pending', 'Cancelled']);

// Dummy data mimicking backend response
const dummyData = [
  {
    id: 1001, customer_name: 'Alice Johnson', employee_name: 'John Doe', store_name: 'Main Street Store', total_amount: '1350.50', status: 'Completed', order_date: '2025-09-22T10:30:00Z',
    items: [
      { product_name: 'Laptop', quantity: 1, price: '1200.50', total: '1200.50' },
      { product_name: 'Mouse', quantity: 2, price: '75.00', total: '150.00' },
    ]
  },
  {
    id: 1002, customer_name: 'Bob Williams', employee_name: 'Jane Smith', store_name: 'Downtown Branch', total_amount: '225.75', status: 'Completed', order_date: '2025-09-21T14:00:00Z',
    items: [
      { product_name: 'Keyboard', quantity: 1, price: '150.75', total: '150.75' },
      { product_name: 'USB Hub', quantity: 1, price: '75.00', total: '75.00' },
    ]
  },
  {
    id: 1003, customer_name: 'Charlie Brown', employee_name: 'John Doe', store_name: 'Main Street Store', total_amount: '85.00', status: 'Pending', order_date: '2025-09-22T11:00:00Z',
    items: [
      { product_name: 'Webcam', quantity: 1, price: '85.00', total: '85.00' },
    ]
  },
  {
    id: 1004, customer_name: 'Diana Prince', employee_name: 'Peter Jones', store_name: 'Uptown Plaza', total_amount: '2400.00', status: 'Cancelled', order_date: '2025-09-20T16:45:00Z',
    items: [
      { product_name: 'Gaming PC', quantity: 1, price: '2400.00', total: '2400.00' },
    ]
  },
];

onMounted(() => {
  bills.value = dummyData;
});

const filteredBills = computed(() => {
    let filtered = bills.value;
    if (filterDate.value) {
        // Basic date filtering, can be improved to a range
        const selectedDate = new Date(filterDate.value).toLocaleDateString();
        filtered = filtered.filter(bill => new Date(bill.order_date).toLocaleDateString() === selectedDate);
    }
    if (filterStatus.value) {
        filtered = filtered.filter(bill => bill.status === filterStatus.value);
    }
    return filtered;
});

const viewBillDetails = (bill) => {
  selectedBill.value = bill;
  billDetailsDialog.value = true;
};

const formatCurrency = (value) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(value);
};

const formatDate = (value) => {
  return new Date(value).toLocaleString('en-US', { year: 'numeric', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
};

const getStatusSeverity = (status) => {
    switch (status) {
        case 'Completed': return 'success';
        case 'Pending': return 'warning';
        case 'Cancelled': return 'danger';
        default: return 'info';
    }
};

const clearFilters = () => {
    filterDate.value = null;
    filterStatus.value = null;
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Bill Management</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <div class="flex items-center gap-2">
                <DatePicker v-model="filterDate" placeholder="Filter by Date" dateFormat="mm/dd/yy" class="w-full md:w-20rem" />
                <Select v-model="filterStatus" :options="statuses" placeholder="Filter by Status" class="w-full md:w-14rem" />
                <Button icon="pi pi-filter-slash" label="Clear" outlined @click="clearFilters" />
            </div>
          </template>
        </Toolbar>

        <DataTable :value="filteredBills" responsiveLayout="scroll" paginator :rows="10">
          <template #empty>
            <div class="text-center py-8">
                <i class="pi pi-file text-4xl text-gray-400 mb-2"></i>
                <h3 class="text-xl font-semibold text-gray-600">No Bills Found</h3>
                <p class="text-gray-500">There are no bills matching the current filters.</p>
            </div>
          </template>

          <Column field="id" header="Bill ID" :sortable="true"></Column>
          <Column field="customer_name" header="Customer" :sortable="true"></Column>
          <Column field="store_name" header="Store" :sortable="true"></Column>
          <Column field="total_amount" header="Total" :sortable="true">
            <template #body="slotProps">{{ formatCurrency(slotProps.data.total_amount) }}</template>
          </Column>
          <Column field="status" header="Status" :sortable="true">
            <template #body="slotProps">
              <Tag :value="slotProps.data.status" :severity="getStatusSeverity(slotProps.data.status)" />
            </template>
          </Column>
          <Column field="order_date" header="Date" :sortable="true">
            <template #body="slotProps">{{ formatDate(slotProps.data.order_date) }}</template>
          </Column>
          <Column headerStyle="width: 10rem; text-align: center" bodyStyle="text-align: center">
             <template #body="slotProps">
                <Button icon="pi pi-eye" severity="info" rounded @click="viewBillDetails(slotProps.data)" v-tooltip.top="'View Details'" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-if="selectedBill" v-model:visible="billDetailsDialog" :style="{width: '600px'}" :header="`Details for Bill #${selectedBill.id}`" :modal="true">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-6 p-2">
            <strong>Customer:</strong> {{ selectedBill.customer_name }}
        </div>
        <div class="field col-6 p-2">
            <strong>Employee:</strong> {{ selectedBill.employee_name }}
        </div>
        <div class="field col-6 p-2">
            <strong>Store:</strong> {{ selectedBill.store_name }}
        </div>
        <div class="field col-6 p-2">
            <strong>Date:</strong> {{ formatDate(selectedBill.order_date) }}
        </div>
        <div class="field col-6 p-2">
            <strong>Status:</strong> <Tag :value="selectedBill.status" :severity="getStatusSeverity(selectedBill.status)" />
        </div>
        <div class="field col-6 p-2 font-bold">
            <strong>Total:</strong> {{ formatCurrency(selectedBill.total_amount) }}
        </div>
        <div class="field col-12 p-2">
            <h4 class="text-xl font-semibold mb-2">Items</h4>
            <DataTable :value="selectedBill.items">
                <Column field="product_name" header="Product"></Column>
                <Column field="quantity" header="Quantity"></Column>
                <Column field="price" header="Price">
                    <template #body="slotProps">{{ formatCurrency(slotProps.data.price) }}</template>
                </Column>
                <Column field="total" header="Total">
                    <template #body="slotProps">{{ formatCurrency(slotProps.data.total) }}</template>
                </Column>
            </DataTable>
        </div>
      </div>
        <template #footer>
            <Button label="Close" icon="pi pi-times" @click="billDetailsDialog = false" text/>
        </template>
    </Dialog>

  </div>
</template>
