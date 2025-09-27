<script setup>
import { onMounted } from 'vue';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Button from 'primevue/button';
import { useRefundStore } from '../../store/refund';

const refundStore = useRefundStore();

onMounted(() => {
  refundStore.fetchRefunds();
});

const formatCurrency = (value) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(value);
};

const formatDate = (value) => {
  return new Date(value).toLocaleDateString('en-US', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
};

const viewRefundDetails = (refund) => {
    // Placeholder for future implementation
    alert(`Viewing details for Refund ID: ${refund.id}`);
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Refunds (Owner)</span>
        </div>
      </template>
      <template #content>
        <DataTable :value="refundStore.refunds" responsiveLayout="scroll">
          <Column field="id" header="Refund ID" :sortable="true"></Column>
          <Column field="order_id" header="Order ID" :sortable="true"></Column>
          <Column field="total_amount" header="Amount" :sortable="true">
            <template #body="slotProps">
                {{ formatCurrency(slotProps.data.total_amount) }}
            </template>
          </Column>
          <Column field="reason" header="Reason"></Column>
          <Column field="created_at" header="Date" :sortable="true">
            <template #body="slotProps">
              {{ formatDate(slotProps.data.created_at) }}
            </template>
          </Column>
          <Column field="employee_id" header="Processed By" :sortable="true"></Column>
          <Column headerStyle="width: 10rem">
             <template #body="slotProps">
                <Button icon="pi pi-eye" label="View" class="mr-2" severity="info" @click="viewRefundDetails(slotProps.data)" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>
  </div>
</template>
