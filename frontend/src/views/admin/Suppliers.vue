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
import Avatar from 'primevue/avatar';
import FloatLabel from 'primevue/floatlabel';
import { useSupplierStore } from '../../store/supplier';

const confirm = useConfirm();
const toast = useToast();
const supplierStore = useSupplierStore();

const selectedSuppliers = ref([]);
const supplierDialog = ref(false);
const submitted = ref(false);
const supplier = ref({});

onMounted(() => {
  supplierStore.fetchSuppliers();
});

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
  if (supplier.value.name && supplier.value.email) {
    try {
      await supplierStore.saveSupplier(supplier.value);
      supplierDialog.value = false;
      supplier.value = {};
      toast.add({ severity: 'success', summary: 'Success', detail: 'Supplier saved successfully', life: 3000 });
    } catch (error) {
      toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to save supplier', life: 3000 });
    }
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
                toast.add({ severity: 'success', summary: 'Success', detail: 'Supplier deleted successfully', life: 3000 });
            } catch (error) {
                toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to delete supplier', life: 3000 });
            }
        }
    });
};

const deleteSelectedSuppliers = () => {
    confirm.require({
        message: `Are you sure you want to delete the ${selectedSuppliers.value.length} selected suppliers?`,
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                await Promise.all(selectedSuppliers.value.map(supp => supplierStore.deleteSupplier(supp.id)));
                selectedSuppliers.value = [];
                toast.add({ severity: 'success', summary: 'Success', detail: 'Selected suppliers deleted successfully', life: 3000 });
            } catch (error) {
                toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to delete selected suppliers', life: 3000 });
            }
        }
    });
};

const formatDate = (value) => {
  return new Date(value).toLocaleString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric', 
    hour: '2-digit', minute: '2-digit'
  });
};

const getInitials = (name) => {
    if (!name) return '';
    return name.split(' ').map(n => n[0]).join('');
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
            <Button label="Delete Selected" icon="pi pi-trash" severity="danger" @click="deleteSelectedSuppliers" :disabled="!selectedSuppliers || !selectedSuppliers.length" />
          </template>
        </Toolbar>

        <DataTable 
          v-model:selection="selectedSuppliers" 
          :value="supplierStore.suppliers" 
          responsiveLayout="scroll"
          paginator :rows="10"
          :rowsPerPageOptions="[10, 20, 50]"
        >
          <template #empty>
            <div class="text-center py-8">
              <i class="pi pi-truck text-4xl text-surface-400 dark:text-surface-500 mb-2"></i>
              <h3 class="text-xl font-semibold text-surface-600 dark:text-surface-300">No Suppliers Found</h3>
              <p class="text-surface-500 dark:text-surface-400">Click the "New" button to add a supplier.</p>
            </div>
          </template>

          <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
          <Column field="name" header="Supplier Name" :sortable="true"></Column>
          <Column field="contact_person" header="Contact" :sortable="true">
            <template #body="slotProps">
              <div class="flex items-center">
                <Avatar :label="getInitials(slotProps.data.contact_person)" class="mr-2" shape="circle" />
                <div>
                  <div>{{ slotProps.data.contact_person }}</div>
                  <div class="text-sm text-surface-500 dark:text-surface-400">{{ slotProps.data.email }}</div>
                </div>
              </div>
            </template>
          </Column>
          <Column field="phone" header="Phone" :sortable="true"></Column>
          
          <Column field="updated_at" header="Last Updated" :sortable="true">
            <template #body="slotProps">
              {{ formatDate(slotProps.data.updated_at) }}
            </template>
          </Column>
          <Column headerStyle="width: 10rem; text-align: center" bodyStyle="text-align: center">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2" severity="success" rounded @click="editSupplier(slotProps.data)" v-tooltip.top="'Edit'" />
                <Button icon="pi pi-trash" severity="danger" rounded @click="confirmDeleteSupplier(slotProps.data)" v-tooltip.top="'Delete'" />
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
                <label for="name">Supplier Name</label>
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
                <InputText id="email" v-model.trim="supplier.email" required="true" :class="{'p-invalid': submitted && !supplier.email}" fluid />
                <label for="email">Email</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !supplier.email">Email is required.</small>
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
