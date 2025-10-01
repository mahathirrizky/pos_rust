<template>
  <div>
    <Card>
      <template #title>
        <span class="text-2xl font-semibold">Reports</span>
      </template>
      <template #content>
        <div class="text-center p-8 bg-surface-50 dark:bg-surface-800 rounded-lg">
          <i class="pi pi-chart-bar text-6xl text-surface-400 dark:text-surface-500 mb-4"></i>
          <h2 class="text-2xl font-semibold text-surface-700 dark:text-surface-200">Reporting Dashboard</h2>
          <p class="text-surface-500 dark:text-surface-400 mt-2">Sales, inventory, and employee performance reports will be available here.</p>
          <p class="text-surface-500 dark:text-surface-400">This feature is under development.</p>
        </div>
      </template>
    </Card>
  </div>
</template>

<script setup>
import Card from 'primevue/card';
</script>
