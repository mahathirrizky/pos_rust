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
import ConfirmDialog from 'primevue/confirmdialog';
import FloatLabel from 'primevue/floatlabel';
import { useCustomerStore } from '../../store/customer';

const confirm = useConfirm();
const toast = useToast();
const customerStore = useCustomerStore();

const selectedCustomers = ref([]);
const customerDialog = ref(false);
const submitted = ref(false);
const customer = ref({});

onMounted(() => {
  customerStore.fetchCustomers();
});

const openNew = () => {
  customer.value = {};
  submitted.value = false;
  customerDialog.value = true;
};

const hideDialog = () => {
  customerDialog.value = false;
  submitted.value = false;
};

const saveCustomer = async () => {
  submitted.value = true;
  if (customer.value.first_name && customer.value.last_name && customer.value.email) {
    try {
      await customerStore.saveCustomer(customer.value);
      customerDialog.value = false;
      customer.value = {};
      toast.add({ severity: 'success', summary: 'Success', detail: 'Customer saved successfully', life: 3000 });
    } catch (error) {
      toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to save customer', life: 3000 });
    }
  }
};

const editCustomer = (cust) => {
  customer.value = { ...cust };
  customerDialog.value = true;
};

const confirmDeleteCustomer = (cust) => {
    confirm.require({
        message: 'Are you sure you want to delete this customer?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                await customerStore.deleteCustomer(cust.id);
                toast.add({ severity: 'success', summary: 'Success', detail: 'Customer deleted successfully', life: 3000 });
            } catch (error) {
                toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to delete customer', life: 3000 });
            }
        }
    });
};

const deleteSelectedCustomers = () => {
    confirm.require({
        message: `Are you sure you want to delete the ${selectedCustomers.value.length} selected customers?`,
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                await Promise.all(selectedCustomers.value.map(cust => customerStore.deleteCustomer(cust.id)));
                selectedCustomers.value = [];
                toast.add({ severity: 'success', summary: 'Success', detail: 'Selected customers deleted successfully', life: 3000 });
            } catch (error) {
                toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to delete selected customers', life: 3000 });
            }
        }
    });
};

const formatDate = (value) => {
  return new Date(value).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric'
  });
};

const getInitials = (firstName, lastName) => {
    if (!firstName || !lastName) return '';
    return `${firstName[0]}${lastName[0]}`;
};

</script>

<template>
  <div>
    
    <ConfirmDialog />
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Customer Management</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New" icon="pi pi-plus" class="mr-2" @click="openNew" />
            <Button label="Delete Selected" icon="pi pi-trash" severity="danger" @click="deleteSelectedCustomers" :disabled="!selectedCustomers || !selectedCustomers.length" />
          </template>
        </Toolbar>

        <div v-if="customerStore.isLoading" class="text-center py-8">
          <i class="pi pi-spin pi-spinner text-4xl text-blue-500"></i>
          <p class="text-lg mt-2">Loading Customers...</p>
        </div>
        <div v-else-if="customerStore.error" class="text-center py-8 text-red-500">
          <p>Error: {{ customerStore.error.message }}</p>
          <Button label="Retry" @click="customerStore.fetchCustomers()" class="mt-4" />
        </div>
        <DataTable 
          v-else
          v-model:selection="selectedCustomers" 
          :value="customerStore.customers" 
          responsiveLayout="scroll"
          paginator :rows="10"
          :rowsPerPageOptions="[10, 20, 50]"
        >
          <template #empty>
            <div class="text-center py-8">
              <i class="pi pi-users text-4xl text-gray-400 mb-2"></i>
              <h3 class="text-xl font-semibold text-gray-600">No Customers Found</h3>
              <p class="text-gray-500">Click the "New" button to add a customer.</p>
            </div>
          </template>

          <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
          <Column header="Name" :sortable="true" field="last_name">
            <template #body="slotProps">
              <div class="flex items-center">
                <Avatar :label="getInitials(slotProps.data.first_name, slotProps.data.last_name)" class="mr-3" shape="circle" size="large" />
                <div>
                  <div class="font-semibold">{{ slotProps.data.first_name }} {{ slotProps.data.last_name }}</div>
                  <div class="text-sm text-gray-500">{{ slotProps.data.email }}</div>
                </div>
              </div>
            </template>
          </Column>
          <Column field="phone" header="Phone" :sortable="true"></Column>
          <Column field="total_orders" header="Total Orders" :sortable="true" headerStyle="text-align: center" bodyStyle="text-align: center"></Column>
          <Column field="created_at" header="Registered On" :sortable="true">
            <template #body="slotProps">
              {{ formatDate(slotProps.data.created_at) }}
            </template>
          </Column>
          <Column headerStyle="width: 10rem; text-align: center" bodyStyle="text-align: center">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2" severity="success" rounded @click="editCustomer(slotProps.data)" v-tooltip.top="'Edit'" />
                <Button icon="pi pi-trash" severity="warning" rounded @click="confirmDeleteCustomer(slotProps.data)" v-tooltip.top="'Delete'" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-model:visible="customerDialog" :style="{width: '450px'}" header="Customer Details" :modal="true" class="p-fluid">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="grid grid-cols-2 gap-4">
          <div class="field col-span-1 p-2">
              <FloatLabel variant="on">
                <InputText id="first_name" v-model.trim="customer.first_name" required="true" autofocus :class="{'p-invalid': submitted && !customer.first_name}" fluid />
                <label for="first_name">First Name</label>
              </FloatLabel>
              <small class="p-error" v-if="submitted && !customer.first_name">First Name is required.</small>
          </div>
          <div class="field col-span-1 p-2">
              <FloatLabel variant="on">
                <InputText id="last_name" v-model.trim="customer.last_name" required="true" :class="{'p-invalid': submitted && !customer.last_name}" fluid />
                <label for="last_name">Last Name</label>
              </FloatLabel>
              <small class="p-error" v-if="submitted && !customer.last_name">Last Name is required.</small>
          </div>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="email" v-model.trim="customer.email" required="true" :class="{'p-invalid': submitted && !customer.email}" fluid />
                <label for="email">Email</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !customer.email">Email is required.</small>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="phone" v-model.trim="customer.phone" fluid />
                <label for="phone">Phone</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Textarea id="address" v-model="customer.address" rows="3" cols="20" fluid />
                <label for="address">Address</label>
            </FloatLabel>
        </div>
      </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="saveCustomer" />
        </template>
    </Dialog>

  </div>
</template>