<script setup>
import { ref, onMounted, watch } from 'vue';
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import InputText from 'primevue/inputtext';
import Password from 'primevue/password';
import Select from 'primevue/select';
import FloatLabel from 'primevue/floatlabel';
import { useEmployeeStore } from '../../store/employee';
import ConfirmDialog from 'primevue/confirmdialog';

const confirm = useConfirm();
const toast = useToast();
const employeeStore = useEmployeeStore();

const selectedStoreId = ref(null);
const selectedEmployees = ref([]);
const employeeDialog = ref(false);
const submitted = ref(false);
const employee = ref({});
const roles = ref(['Cashier', 'InventoryManager', 'StoreManager']); // Admin can create these roles

onMounted(() => {
  employeeStore.fetchEmployees({ roles_to_exclude: 'Owner,Admin' });
  employeeStore.fetchStores();
});

watch(selectedStoreId, () => {
  employeeStore.fetchEmployees({ store_id: selectedStoreId.value, roles_to_exclude: 'Owner,Admin' });
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

const saveEmployee = async () => {
  submitted.value = true;
  if (!employee.value.first_name || !employee.value.email || !employee.value.role) {
    return;
  }

  if (!employee.value.id && !employee.value.password) {
      return;
  }

  try {
    await employeeStore.saveEmployee(employee.value);
    const isUpdate = !!employee.value.id;
    toast.add({ severity: 'success', summary: 'Success', detail: `Employee ${isUpdate ? 'updated' : 'created'} successfully.`, life: 3000 });
    employeeDialog.value = false;
  } catch (err) {
    toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
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
        accept: async () => {
            try {
                await employeeStore.deleteEmployee(prod);
                toast.add({ severity: 'success', summary: 'Confirmed', detail: 'Employee deleted', life: 3000 });
            } catch (err) {
                toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
            }
        },
    });
};

const deleteSelectedEmployees = () => {
    confirm.require({
        message: 'Are you sure you want to delete the selected employees?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                await Promise.all(selectedEmployees.value.map(emp => employeeStore.deleteEmployee(emp)));
                toast.add({ severity: 'success', summary: 'Confirmed', detail: 'Selected employees deleted', life: 3000 });
                selectedEmployees.value = [];
            } catch (err) {
                toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to delete selected employees.', life: 3000 });
            }
        },
    });
};

const formatDate = (value) => {
  if (!value) return '';
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
          <span class="text-2xl font-semibold">Employee Management (Admin)</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New" icon="pi pi-plus" class="mr-2" @click="openNew" />
            <Button label="Delete" icon="pi pi-trash" severity="danger" @click="deleteSelectedEmployees" :disabled="!selectedEmployees || !selectedEmployees.length" />
          </template>
          <template #end>
            <Select v-model="selectedStoreId" :options="employeeStore.stores" optionLabel="label" optionValue="value" placeholder="Filter by Store" class="w-full md:w-14rem" />
          </template>
        </Toolbar>

        <DataTable :value="employeeStore.employees" responsiveLayout="scroll">
          <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
          <Column field="id" header="ID"></Column>
          <Column field="first_name" header="First Name"></Column>
          <Column field="last_name" header="Last Name"></Column>
          <Column field="email" header="Email"></Column>
          <Column field="role" header="Role"></Column>
          <Column field="store_id" header="Store ID"></Column>
          <Column field="created_at" header="Created At">
            <template #body="slotProps">
              {{ formatDate(slotProps.data.created_at) }}
            </template>
          </Column>
          <Column headerStyle="width: 10rem">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2" @click="editEmployee(slotProps.data)" />
                <Button icon="pi pi-trash" severity="danger" @click="confirmDeleteEmployee(slotProps.data)" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-model:visible="employeeDialog" :style="{width: '450px'}" header="Employee Details" :modal="true" class="p-fluid">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="firstname" v-model.trim="employee.first_name" required autofocus fluid />
                <label for="firstname">First Name</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="lastname" v-model.trim="employee.last_name" required fluid />
                <label for="lastname">Last Name</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="email" v-model.trim="employee.email" required fluid />
                <label for="email">Email</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Select id="role" v-model="employee.role" :options="roles" placeholder="Select a Role" required fluid />
                <label for="role">Role</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Select id="store" v-model="employee.store_id" :options="employeeStore.stores.filter(s => s.value !== null)" optionLabel="label" optionValue="value" placeholder="Select a Store" required fluid />
                <label for="store">Store</label>
            </FloatLabel>
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