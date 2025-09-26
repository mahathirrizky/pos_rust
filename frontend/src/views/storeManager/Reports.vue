<template>
  <div>
    <Card>
      <template #title>
        <span class="text-2xl font-semibold">Reports</span>
      </template>
      <template #content>
        <div class="text-center p-8 bg-gray-50 rounded-lg">
          <i class="pi pi-chart-bar text-6xl text-gray-400 mb-4"></i>
          <h2 class="text-2xl font-semibold text-gray-700">Reporting Dashboard</h2>
          <p class="text-gray-500 mt-2">Sales, inventory, and employee performance reports will be available here.</p>
          <p class="text-gray-500">This feature is under development.</p>
        </div>
      </template>
    </Card>
  </div>
</template>

<script setup>
import Card from 'primevue/card';
</script>
