<script setup>
import { ref, onMounted, watch } from 'vue';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import Select from 'primevue/select';
import { useEmployeeStore } from '../../store/employee';

const employeeStore = useEmployeeStore();

const selectedStoreId = ref(null);

onMounted(() => {
  employeeStore.fetchEmployees({ roles_to_exclude: 'Owner' });
  employeeStore.fetchStores();
});

watch(selectedStoreId, (newStoreId) => {
  const filters = { roles_to_exclude: 'Owner' };
  if (newStoreId) {
    filters.store_id = newStoreId;
  }
  employeeStore.fetchEmployees(filters);
});

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
    
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Employee Management (Owner)</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          
          <template #end>
            <Select v-model="selectedStoreId" :options="employeeStore.stores" optionLabel="label" optionValue="value" placeholder="Filter by Store" class="w-full md:w-14rem" />
          </template>
        </Toolbar>

        <DataTable :value="employeeStore.employees" responsiveLayout="scroll">
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
        </DataTable>
      </template>
    </Card>

    

  </div>
</template>