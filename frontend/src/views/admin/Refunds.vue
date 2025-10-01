<script setup>
import { ref, onMounted, computed } from 'vue';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import Tag from 'primevue/tag';
import Toolbar from 'primevue/toolbar';
import DatePicker from 'primevue/datepicker';
import { useRefundStore } from '../../store/refund';

const refundStore = useRefundStore();
const selectedRefund = ref(null);
const refundDetailsDialog = ref(false);
const filterDate = ref();

onMounted(() => {
  refundStore.fetchRefunds();
});

const filteredRefunds = computed(() => {
    if (!filterDate.value) return refundStore.refunds;
    const selectedDate = new Date(filterDate.value).toLocaleDateString();
    return refundStore.refunds.filter(refund => new Date(refund.created_at).toLocaleDateString() === selectedDate);
});

const formatCurrency = (value) => {
    return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR' }).format(value);
};

const formatDate = (value) => {
  return new Date(value).toLocaleString('en-US', { year: 'numeric', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
};

const viewRefundDetails = (refund) => {
    selectedRefund.value = refund;
    refundDetailsDialog.value = true;
};

const getStatusSeverity = (status) => {
    switch (status) {
        case 'Completed': return 'success';
        case 'Processing': return 'warning';
        case 'Rejected': return 'danger';
        default: return 'info';
    }
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Refund Management</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
            <template #start>
                <DatePicker v-model="filterDate" placeholder="Filter by Date" dateFormat="mm/dd/yy" />
            </template>
        </Toolbar>

        <DataTable :value="filteredRefunds" responsiveLayout="scroll" paginator :rows="10">
          <template #empty>
            <div class="text-center py-8">
                <i class="pi pi-replay text-4xl text-surface-400 dark:text-surface-500 mb-2"></i>
                <h3 class="text-xl font-semibold text-surface-600 dark:text-surface-300">No Refunds Found</h3>
                <p class="text-surface-500 dark:text-surface-400">No refunds match the current filters.</p>
            </div>
          </template>

          <Column field="id" header="Refund ID" :sortable="true"></Column>
          <Column field="order_id" header="Original Order ID" :sortable="true"></Column>
          <Column field="customer_name" header="Customer" :sortable="true"></Column>
          <Column field="total_amount" header="Amount" :sortable="true">
            <template #body="slotProps">{{ formatCurrency(slotProps.data.total_amount) }}</template>
          </Column>
          <Column field="status" header="Status" :sortable="true">
            <template #body="slotProps">
              <Tag :value="slotProps.data.status" :severity="getStatusSeverity(slotProps.data.status)" />
            </template>
          </Column>
          <Column field="created_at" header="Date" :sortable="true">
            <template #body="slotProps">{{ formatDate(slotProps.data.created_at) }}</template>
          </Column>
          <Column headerStyle="width: 10rem; text-align: center" bodyStyle="text-align: center">
             <template #body="slotProps">
                <Button icon="pi pi-eye" severity="info" rounded @click="viewRefundDetails(slotProps.data)" v-tooltip.top="'View Details'" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-if="selectedRefund" v-model:visible="refundDetailsDialog" :style="{width: '600px'}" :header="`Details for Refund #${selectedRefund.id}`" :modal="true">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-6 p-2">
            <strong>Original Order ID:</strong> #{{ selectedRefund.order_id }}
        </div>
        <div class="field col-6 p-2">
            <strong>Customer:</strong> {{ selectedRefund.customer_name }}
        </div>
        <div class="field col-6 p-2">
            <strong>Processed By:</strong> {{ selectedRefund.employee_name }}
        </div>
        <div class="field col-6 p-2">
            <strong>Store:</strong> {{ selectedRefund.store_name }}
        </div>
        <div class="field col-6 p-2">
            <strong>Date:</strong> {{ formatDate(selectedRefund.created_at) }}
        </div>
        <div class="field col-6 p-2">
            <strong>Status:</strong> <Tag :value="selectedRefund.status" :severity="getStatusSeverity(selectedRefund.status)" />
        </div>
        <div class="field col-12 p-2">
            <strong>Reason:</strong> {{ selectedRefund.reason }}
        </div>
        <div class="field col-12 p-2 font-bold border-t pt-2 mt-2">
            <strong>Total Refunded:</strong> {{ formatCurrency(selectedRefund.total_amount) }}
        </div>
        <div class="field col-12 p-2">
            <h4 class="text-xl font-semibold mb-2">Refunded Items</h4>
            <DataTable :value="selectedRefund.items">
                <Column field="product_name" header="Product"></Column>
                <Column field="quantity" header="Quantity"></Column>
                <Column field="amount" header="Amount">
                    <template #body="slotProps">{{ formatCurrency(slotProps.data.amount) }}</template>
                </Column>
            </DataTable>
        </div>
      </div>
        <template #footer>
            <Button label="Close" icon="pi pi-times" @click="refundDetailsDialog = false" text/>
        </template>
    </Dialog>

  </div>
</template>
