<script setup>
import { ref, onMounted } from 'vue';
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import InputText from 'primevue/inputtext';
import Textarea from 'primevue/textarea';
import FloatLabel from 'primevue/floatlabel';
import { useSupplierStore } from '../../store/supplier'; // Import store

// Initialize stores and services
const confirm = useConfirm();
const toast = useToast();
const supplierStore = useSupplierStore();

// Component state
const selectedSuppliers = ref([]);
const supplierDialog = ref(false);
const submitted = ref(false);
const supplier = ref({});

// Fetch data on mount
onMounted(() => {
  supplierStore.fetchSuppliers();
});

// CRUD Operations
const openNew = () => {
  supplier.value = {};
  submitted.value = false;
  supplierDialog.value = true;
};

const hideDialog = () => {
  supplierDialog.value = false;
  submitted.value = false;
};

const saveSupplier = async () => {
  submitted.value = true;
  if (!supplier.value.name) {
    return; // Basic validation
  }

  try {
    await supplierStore.saveSupplier(supplier.value);
    const isUpdate = !!supplier.value.id;
    toast.add({ severity: 'success', summary: 'Success', detail: `Supplier ${isUpdate ? 'updated' : 'created'} successfully.`, life: 3000 });
    supplierDialog.value = false;
    supplier.value = {};
  } catch (err) {
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to save supplier.', life: 3000 });
  }
};

const editSupplier = (supp) => {
  supplier.value = { ...supp };
  supplierDialog.value = true;
};

const confirmDeleteSupplier = (supp) => {
    confirm.require({
        message: 'Are you sure you want to delete this supplier?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                await supplierStore.deleteSupplier(supp.id);
                toast.add({ severity: 'success', summary: 'Confirmed', detail: 'Supplier deleted', life: 3000 });
            } catch (err) {
                toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to delete supplier.', life: 3000 });
            }
        },
    });
};

const deleteSelectedSuppliers = () => {
    confirm.require({
        message: `Are you sure you want to delete the ${selectedSuppliers.value.length} selected suppliers?`,
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                await Promise.all(selectedSuppliers.value.map(s => supplierStore.deleteSupplier(s.id)));
                toast.add({ severity: 'success', summary: 'Confirmed', detail: 'Selected suppliers deleted', life: 3000 });
                selectedSuppliers.value = [];
            } catch (err) {
                toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to delete selected suppliers.', life: 3000 });
            }
        },
    });
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Supplier Management</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New" icon="pi pi-plus" class="mr-2" @click="openNew" />
            <Button label="Delete" icon="pi pi-trash" severity="danger" @click="deleteSelectedSuppliers" :disabled="!selectedSuppliers || !selectedSuppliers.length" />
          </template>
        </Toolbar>

        <DataTable v-model:selection="selectedSuppliers" :value="supplierStore.suppliers" responsiveLayout="scroll">
          <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
          <Column field="id" header="ID" :sortable="true"></Column>
          <Column field="name" header="Name" :sortable="true"></Column>
          <Column field="contact_person" header="Contact Person" :sortable="true"></Column>
          <Column field="email" header="Email" :sortable="true"></Column>
          <Column field="phone" header="Phone" :sortable="true"></Column>
          <Column headerStyle="width: 10rem">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2" severity="success" rounded @click="editSupplier(slotProps.data)" />
                <Button icon="pi pi-trash" severity="danger" rounded @click="confirmDeleteSupplier(slotProps.data)" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-model:visible="supplierDialog" :style="{width: '450px'}" header="Supplier Details" :modal="true" class="p-fluid">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="name" v-model.trim="supplier.name" required="true" autofocus :class="{'p-invalid': submitted && !supplier.name}" fluid />
                <label for="name">Name</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !supplier.name">Name is required.</small>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="contact_person" v-model.trim="supplier.contact_person" fluid />
                <label for="contact_person">Contact Person</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="email" v-model.trim="supplier.email" fluid />
                <label for="email">Email</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="phone" v-model.trim="supplier.phone" fluid />
                <label for="phone">Phone</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Textarea id="address" v-model="supplier.address" rows="3" cols="20" fluid />
                <label for="address">Address</label>
            </FloatLabel>
        </div>
      </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="saveSupplier" />
        </template>
    </Dialog>

  </div>
</template>
