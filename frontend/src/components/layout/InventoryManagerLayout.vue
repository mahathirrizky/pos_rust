<script setup>
import { ref, onMounted } from 'vue';
import Sidebar from './Sidebar.vue';

const employee = ref(null);

const loadDummyEmployee = () => {
  employee.value = {
    first_name: 'Dummy',
    last_name: 'InventoryManager',
    role: 'InventoryManager'
  };
};

onMounted(() => {
  loadDummyEmployee();
});
</script>

<template>
  <div class="flex bg-gray-50">
    <Sidebar :employee="employee" />
    <main class="flex-grow p-6 h-screen overflow-auto">
      <router-view />
    </main>
    <div v-if="employee" class="absolute bottom-0 right-0 p-4 text-sm text-gray-600">
      Logged in as: {{ employee.first_name }} {{ employee.last_name }} ({{ employee.role }})
    </div>
  </div>
</template>