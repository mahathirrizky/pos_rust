<script setup>
import { ref, onMounted } from 'vue';
import { useToast } from "primevue/usetoast";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import InputNumber from 'primevue/inputnumber';
import FloatLabel from 'primevue/floatlabel';
import { useInventoryStore } from '../../store/inventory';

const toast = useToast();
const inventoryStore = useInventoryStore();

const inventoryDialog = ref(false);
const submitted = ref(false);
const inventoryItem = ref({});

onMounted(() => {
  inventoryStore.fetchInventoryReport();
});

const openEditDialog = (item) => {
  inventoryItem.value = { ...item };
  inventoryDialog.value = true;
};

const hideDialog = () => {
  inventoryDialog.value = false;
  submitted.value = false;
};

const saveInventory = async () => {
  submitted.value = true;
  if (inventoryItem.value.quantity === null || inventoryItem.value.quantity < 0) {
    return;
  }

  try {
    await inventoryStore.updateInventoryItem(inventoryItem.value);
    toast.add({ severity: 'success', summary: 'Success', detail: 'Inventory updated successfully.', life: 3000 });
    inventoryDialog.value = false;
  } catch (err) {
    toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
  }
};

const formatDate = (value) => {
  if (!value) return '';
  return new Date(value).toLocaleString();
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Inventory List</span>
        </div>
      </template>
      <template #content>
        <div v-if="!inventoryStore.inventoryReport" class="text-center py-8">
          <i class="pi pi-spin pi-spinner text-4xl text-blue-500"></i>
          <p>Loading inventory...</p>
        </div>
        <div v-else-if="inventoryStore.inventoryReport.error" class="text-center py-8 text-red-500">
          <p>{{ inventoryStore.inventoryReport.error }}</p>
          <Button label="Retry" @click="inventoryStore.fetchInventoryReport" class="mt-4" />
        </div>

        <DataTable v-else :value="inventoryStore.inventoryReport.items" responsiveLayout="scroll">
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
          <Column headerStyle="width: 10rem">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2 p-button-rounded p-button-success" @click="openEditDialog(slotProps.data)" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-model:visible="inventoryDialog" :style="{width: '450px'}" header="Update Inventory" :modal="true" class="p-fluid">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputNumber id="quantity" v-model="inventoryItem.quantity" required="true" :class="{'p-invalid': submitted && (inventoryItem.quantity === null || inventoryItem.quantity < 0)}" variant="filled" class="w-full" />
                <label for="quantity">Quantity</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && (inventoryItem.quantity === null || inventoryItem.quantity < 0)">Quantity is required and must be non-negative.</small>
        </div>
      </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="saveInventory" />
        </template>
    </Dialog>

  </div>
</template>