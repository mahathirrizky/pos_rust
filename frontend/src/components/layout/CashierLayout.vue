<script setup>
import { ref, onMounted } from 'vue';
import Sidebar from './Sidebar.vue';
import Toast from 'primevue/toast';
// Removed useToast import

const employee = ref(null);
// Removed toast instance

const loadDummyEmployee = () => {
  employee.value = {
    first_name: 'Dummy',
    last_name: 'Cashier',
    role: 'Guest'
  };
};

onMounted(() => {
  loadDummyEmployee();
});
</script>

<template>
  <div class="flex bg-gray-50">
    <Toast />
    <Sidebar />
    <main class="flex-grow p-6 h-screen overflow-auto">
      <router-view />
    </main>
    <!-- Display employee name at the bottom -->
    <div v-if="employee" class="absolute bottom-0 right-0 p-4 text-sm text-gray-600">
      Logged in as: {{ employee.first_name }} {{ employee.last_name }} ({{ employee.role }})
    </div>
  </div>
</template>
