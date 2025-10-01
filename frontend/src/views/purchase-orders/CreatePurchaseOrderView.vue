<script setup>
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useToast } from 'primevue/usetoast';
import { usePurchaseOrderStore } from '../../store/purchaseOrder';
import { useSupplierStore } from '../../store/supplier';
import { useStoreStore } from '../../store/store';

import Card from 'primevue/card';
import Button from 'primevue/button';
import Dropdown from 'primevue/dropdown';
import FloatLabel from 'primevue/floatlabel';

const router = useRouter();
const toast = useToast();
const purchaseOrderStore = usePurchaseOrderStore();
const supplierStore = useSupplierStore();
const storeStore = useStoreStore();

const po = ref({});
const submitted = ref(false);

onMounted(() => {
  supplierStore.fetchSuppliers();
  storeStore.fetchStores();
});

const savePurchaseOrder = async () => {
  submitted.value = true;
  if (!po.value.supplier_id || !po.value.store_id) {
    toast.add({ severity: 'error', summary: 'Validation Error', detail: 'Please select a supplier and a store.', life: 3000 });
    return;
  }

  try {
    const newPo = await purchaseOrderStore.createPurchaseOrder(po.value);
    toast.add({ severity: 'success', summary: 'Success', detail: `Purchase Order #${newPo.id} created successfully!`, life: 3000 });
    
    // Redirect to the detail page to add items
    const currentPath = router.currentRoute.value.path;
    const detailPath = currentPath.replace('/new', `/${newPo.id}`);
    router.push(detailPath);

  } catch (err) {
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to create purchase order.', life: 3000 });
  }
};

const goBack = () => {
    router.back();
}

</script>

<template>
  <Card>
    <template #title>
      <div class="flex justify-between items-center">
        <span class="text-2xl font-semibold">Create New Purchase Order</span>
      </div>
    </template>
    <template #content>
      <div class="p-fluid grid grid-cols-1 md:grid-cols-2 gap-4">
        
        <div class="field">
          <FloatLabel>
            <Dropdown id="supplier" v-model="po.supplier_id" :options="supplierStore.suppliers" optionLabel="name" optionValue="id" placeholder="Select a Supplier" :class="{'p-invalid': submitted && !po.supplier_id}"></Dropdown>
            <label for="supplier">Supplier</label>
          </FloatLabel>
        </div>

        <div class="field">
          <FloatLabel>
            <Select id="store" v-model="po.store_id" :options="storeStore.stores" optionLabel="name" optionValue="id" placeholder="Select a Store" :class="{'p-invalid': submitted && !po.store_id}"></Select>
            <label for="store">Store</label>
          </FloatLabel>
        </div>

      </div>
    </template>
    <template #footer>
        <div class="flex justify-end gap-2">
            <Button label="Cancel" icon="pi pi-times" @click="goBack" severity="secondary"/>
            <Button label="Save and Add Items" icon="pi pi-check" severity="success" @click="savePurchaseOrder" />
        </div>
    </template>
  </Card>
</template>
