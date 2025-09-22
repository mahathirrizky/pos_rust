<template>
  <div class="flex flex-col items-center justify-center min-h-screen bg-gray-100 p-4">
    <h1 class="text-4xl font-bold text-gray-800 mb-8">Welcome to the POS System</h1>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <!-- Cashier App Card -->
      <router-link to="/cashier" class="block">
        <div class="bg-white rounded-lg shadow-lg hover:shadow-xl transition-shadow duration-300 p-6 text-center cursor-pointer">
          <i class="pi pi-shopping-cart text-6xl text-blue-500 mb-4"></i>
          <h2 class="text-2xl font-semibold text-gray-700 mb-2">Cashier App</h2>
          <p class="text-gray-600">Process sales, manage orders, and handle customer transactions.</p>
        </div>
      </router-link>

      <!-- Inventory Manager App Card -->
      <router-link to="/inventory-manager" class="block">
        <div class="bg-white rounded-lg shadow-lg hover:shadow-xl transition-shadow duration-300 p-6 text-center cursor-pointer">
          <i class="pi pi-box text-6xl text-green-500 mb-4"></i>
          <h2 class="text-2xl font-semibold text-gray-700 mb-2">Inventory Manager</h2>
          <p class="text-gray-600">Manage product stock, track inventory levels, and handle suppliers.</p>
        </div>
      </router-link>
    </div>
  </div>
</template>

<script setup>
// No specific script logic needed for this simple landing page
</script>

<style scoped>
/* Add any specific styles here if needed */
</style>