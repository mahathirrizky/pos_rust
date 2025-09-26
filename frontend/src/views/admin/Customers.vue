<script setup>
import { ref, onMounted } from 'vue';
import { useConfirm } from "primevue/useconfirm";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import InputText from 'primevue/inputtext';
import Textarea from 'primevue/textarea';
import Avatar from 'primevue/avatar';

const confirm = useConfirm();
const customers = ref([]);
const selectedCustomers = ref([]);
const customerDialog = ref(false);
const submitted = ref(false);
const customer = ref({});

// Dummy data mimicking backend response
const dummyData = [
    {
        id: 1,
        first_name: 'Alice',
        last_name: 'Johnson',
        email: 'alice.j@example.com',
        phone: '555-100-2000',
        address: '101 Pine St, Anytown, USA',
        total_orders: 5,
        created_at: '2024-03-01T09:00:00Z',
        updated_at: '2025-09-22T09:00:00Z',
    },
    {
        id: 2,
        first_name: 'Bob',
        last_name: 'Williams',
        email: 'bob.w@example.com',
        phone: '555-300-4000',
        address: '202 Maple Ave, Cityville, USA',
        total_orders: 12,
        created_at: '2024-04-10T14:00:00Z',
        updated_at: '2025-08-15T14:00:00Z',
    },
    {
        id: 3,
        first_name: 'Charlie',
        last_name: 'Brown',
        email: 'charlie.b@example.com',
        phone: '555-500-6000',
        address: '303 Oak Ln, Metropolis, USA',
        total_orders: 2,
        created_at: '2025-01-20T11:20:00Z',
        updated_at: '2025-09-10T11:20:00Z',
    },
    {
        id: 4,
        first_name: 'Diana',
        last_name: 'Prince',
        email: 'diana.p@example.com',
        phone: '555-700-8000',
        address: '404 Justice Rd, Themyscira, USA',
        total_orders: 25,
        created_at: '2025-05-30T18:00:00Z',
        updated_at: '2025-09-21T18:00:00Z',
    },
];

onMounted(() => {
  customers.value = dummyData;
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

const saveCustomer = () => {
  submitted.value = true;
  if (customer.value.first_name && customer.value.last_name && customer.value.email) {
    if (customer.value.id) {
      const index = customers.value.findIndex(c => c.id === customer.value.id);
      if (index > -1) {
        customers.value[index] = { ...customers.value[index], ...customer.value, updated_at: new Date().toISOString() };
      }
    } else {
      customer.value.id = Math.max(0, ...customers.value.map(c => c.id)) + 1;
      customer.value.total_orders = 0;
      customer.value.created_at = new Date().toISOString();
      customer.value.updated_at = new Date().toISOString();
      customers.value.push(customer.value);
    }
    customerDialog.value = false;
    customer.value = {};
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
        accept: () => {
            deleteCustomer(cust);
        }
    });
};

const deleteCustomer = (cust) => {
    customers.value = customers.value.filter(c => c.id !== cust.id);
};

const deleteSelectedCustomers = () => {
    confirm.require({
        message: `Are you sure you want to delete the ${selectedCustomers.value.length} selected customers?`,
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: () => {
            customers.value = customers.value.filter(c => !selectedCustomers.value.includes(c));
            selectedCustomers.value = [];
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

        <DataTable 
          v-model:selection="selectedCustomers" 
          :value="customers" 
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