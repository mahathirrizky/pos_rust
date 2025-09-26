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
import Select from 'primevue/select';
import Password from 'primevue/password';
import FloatLabel from 'primevue/floatlabel';

const confirm = useConfirm();
const employees = ref([]);
const selectedEmployees = ref([]);
const employeeDialog = ref(false);
const submitted = ref(false);
const employee = ref({});
const roles = ref(['Cashier', 'InventoryManager', 'StoreManager']);

// Dummy data mimicking backend response
const dummyData = [
  {
    id: 1,
    first_name: 'John',
    last_name: 'Doe',
    email: 'john.doe@example.com',
    phone: '123-456-7890',
    store_id: 1,
    role: 'Cashier',
    created_at: '2025-09-21T10:00:00Z',
    updated_at: '2025-09-21T10:00:00Z',
  },
  {
    id: 2,
    first_name: 'Jane',
    last_name: 'Smith',
    email: 'jane.smith@example.com',
    phone: '098-765-4321',
    store_id: 1,
    role: 'Cashier',
    created_at: '2025-09-20T11:30:00Z',
    updated_at: '2025-09-21T12:00:00Z',
  },
  {
    id: 3,
    first_name: 'Peter',
    last_name: 'Jones',
    email: 'peter.jones@example.com',
    phone: null,
    store_id: 1,
    role: 'InventoryManager',
    created_at: '2025-09-19T09:00:00Z',
    updated_at: '2025-09-22T08:45:00Z',
  },
];

onMounted(() => {
  employees.value = dummyData;
});

const openNew = () => {
  employee.value = {};
  submitted.value = false;
  employeeDialog.value = true;
};

const hideDialog = () => {
  employeeDialog.value = false;
  submitted.value = false;
};

const saveEmployee = () => {
  submitted.value = true;
  if (employee.value.first_name && employee.value.last_name && employee.value.email && employee.value.role) {
    if (employee.value.id) {
      const index = employees.value.findIndex(e => e.id === employee.value.id);
      if (index > -1) {
        employees.value[index] = { ...employees.value[index], ...employee.value };
      }
    } else {
      employee.value.id = Math.max(...employees.value.map(e => e.id)) + 1;
      employee.value.created_at = new Date().toISOString();
      employee.value.updated_at = new Date().toISOString();
      employee.value.store_id = 1;
      employees.value.push(employee.value);
    }
    employeeDialog.value = false;
    employee.value = {};
  }
};

const editEmployee = (prod) => {
  employee.value = { ...prod };
  employeeDialog.value = true;
};

const confirmDeleteEmployee = (prod) => {
    confirm.require({
        message: 'Are you sure you want to delete this employee?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: () => {
            deleteEmployee(prod);
        }
    });
};

const deleteEmployee = (prod) => {
    employees.value = employees.value.filter(e => e.id !== prod.id);
};

const deleteSelectedEmployees = () => {
    confirm.require({
        message: 'Are you sure you want to delete the selected employees?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: () => {
            employees.value = employees.value.filter(e => !selectedEmployees.value.includes(e));
            selectedEmployees.value = [];
        }
    });
};

const formatDate = (value) => {
  return new Date(value).toLocaleDateString('en-US', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
  });
};

</script>

<template>
  <div>
    <ConfirmDialog></ConfirmDialog>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Employee Management</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New" icon="pi pi-plus" class="mr-2" @click="openNew" />
            <Button label="Delete" icon="pi pi-trash" severity="danger" @click="deleteSelectedEmployees" :disabled="!selectedEmployees || !selectedEmployees.length" />
          </template>
        </Toolbar>

        <DataTable v-model:selection="selectedEmployees" :value="employees" responsiveLayout="scroll">
          <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
          <Column field="id" header="ID" :sortable="true"></Column>
          <Column field="first_name" header="First Name" :sortable="true"></Column>
          <Column field="last_name" header="Last Name" :sortable="true"></Column>
          <Column field="email" header="Email" :sortable="true"></Column>
          <Column field="role" header="Role" :sortable="true"></Column>
          <Column field="created_at" header="Created At" :sortable="true">
            <template #body="slotProps">
              {{ formatDate(slotProps.data.created_at) }}
            </template>
          </Column>
          <Column headerStyle="width: 10rem">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2" severity="success" rounded @click="editEmployee(slotProps.data)" />
                <Button icon="pi pi-trash" severity="warning" rounded @click="confirmDeleteEmployee(slotProps.data)" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-model:visible="employeeDialog" :style="{width: '450px'}" header="Employee Details" :modal="true" class="p-fluid">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="firstname" v-model.trim="employee.first_name" required="true" autofocus :class="{'p-invalid': submitted && !employee.first_name}" fluid />
                <label for="firstname">First Name</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !employee.first_name">First Name is required.</small>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="lastname" v-model.trim="employee.last_name" required="true" :class="{'p-invalid': submitted && !employee.last_name}" fluid />
                <label for="lastname">Last Name</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !employee.last_name">Last Name is required.</small>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="email" v-model.trim="employee.email" required="true" :class="{'p-invalid': submitted && !employee.email}" fluid />
                <label for="email">Email</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !employee.email">Email is required.</small>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Select id="role" v-model="employee.role" :options="roles" placeholder="Select a Role" required="true" :class="{'p-invalid': submitted && !employee.role}" fluid />
                <label for="role">Role</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !employee.role">Role is required.</small>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Password id="password" v-model="employee.password" :feedback="false" fluid />
                <label for="password">Password</label>
            </FloatLabel>
        </div>
      </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="saveEmployee" />
        </template>
    </Dialog>

  </div>
</template>
