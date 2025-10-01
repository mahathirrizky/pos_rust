<script setup>
import { onMounted } from 'vue';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import { useProductStore } from '../../store/product';

const productStore = useProductStore();

onMounted(() => {
  productStore.fetchProducts();
});

const formatCurrency = (value) => {
    return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR' }).format(value);
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Product Management (Owner)</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <!-- Owner can only view products, no CRUD operations from here -->
          </template>
        </Toolbar>

        <DataTable :value="productStore.products" responsiveLayout="scroll">
          <Column field="id" header="ID" :sortable="true"></Column>
          <Column field="name" header="Name" :sortable="true"></Column>
          <Column field="sku" header="SKU" :sortable="true"></Column>
          <Column field="price" header="Price" :sortable="true">
            <template #body="slotProps">
                {{ formatCurrency(slotProps.data.price) }}
            </template>
          </Column>
          <Column field="category_name" header="Category" :sortable="true"></Column>
          <Column field="supplier_name" header="Supplier" :sortable="true"></Column>
        </DataTable>
      </template>
    </Card>
  </div>
</template>