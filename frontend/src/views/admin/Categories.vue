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
import Tag from 'primevue/tag';

const confirm = useConfirm();
const categories = ref([]);
const selectedCategories = ref([]);
const categoryDialog = ref(false);
const submitted = ref(false);
const category = ref({});

// Dummy data mimicking backend response with more variety
const dummyData = [
    {
        id: 1,
        name: 'Electronics',
        description: 'Gadgets, computers, and related accessories.',
        product_count: 12,
        created_at: '2025-01-15T10:30:00Z',
        updated_at: '2025-09-12T14:00:00Z',
    },
    {
        id: 2,
        name: 'Home & Kitchen',
        description: 'Items for home, kitchen, and garden.',
        product_count: 45,
        created_at: '2025-02-20T11:00:00Z',
        updated_at: '2025-08-30T18:20:00Z',
    },
    {
        id: 3,
        name: 'Apparel',
        description: 'Clothing, shoes, and accessories for all ages.',
        product_count: 78,
        created_at: '2025-03-10T09:00:00Z',
        updated_at: '2025-09-21T10:05:00Z',
    },
    {
        id: 4,
        name: 'Books',
        description: 'Physical books, e-books, and audiobooks.',
        product_count: 150,
        created_at: '2025-04-05T14:15:00Z',
        updated_at: '2025-09-01T11:00:00Z',
    },
];

onMounted(() => {
  categories.value = dummyData;
});

const openNew = () => {
  category.value = {};
  submitted.value = false;
  categoryDialog.value = true;
};

const hideDialog = () => {
  categoryDialog.value = false;
  submitted.value = false;
};

const saveCategory = () => {
  submitted.value = true;
  if (category.value.name) {
    if (category.value.id) {
      const index = categories.value.findIndex(c => c.id === category.value.id);
      if (index > -1) {
        categories.value[index] = { ...categories.value[index], ...category.value, updated_at: new Date().toISOString() };
      }
    } else {
      category.value.id = Math.max(0, ...categories.value.map(c => c.id)) + 1;
      category.value.product_count = 0; // New categories have no products
      category.value.created_at = new Date().toISOString();
      category.value.updated_at = new Date().toISOString();
      categories.value.push(category.value);
    }
    categoryDialog.value = false;
    category.value = {};
  }
};

const editCategory = (cat) => {
  category.value = { ...cat };
  categoryDialog.value = true;
};

const confirmDeleteCategory = (cat) => {
    confirm.require({
        message: 'Are you sure you want to delete this category?',
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: () => {
            deleteCategory(cat);
        }
    });
};

const deleteCategory = (cat) => {
    categories.value = categories.value.filter(c => c.id !== cat.id);
};

const deleteSelectedCategories = () => {
    confirm.require({
        message: `Are you sure you want to delete the ${selectedCategories.value.length} selected categories?`,
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: () => {
            categories.value = categories.value.filter(c => !selectedCategories.value.includes(c));
            selectedCategories.value = [];
        }
    });
};

const formatDate = (value) => {
  return new Date(value).toLocaleString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric', 
    hour: '2-digit', minute: '2-digit'
  });
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Category Management</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New" icon="pi pi-plus" class="mr-2" @click="openNew" />
            <Button label="Delete Selected" icon="pi pi-trash" severity="danger" @click="deleteSelectedCategories" :disabled="!selectedCategories || !selectedCategories.length" />
          </template>
        </Toolbar>

        <DataTable 
          v-model:selection="selectedCategories" 
          :value="categories" 
          responsiveLayout="scroll"
          paginator :rows="10"
          :rowsPerPageOptions="[10, 20, 50]"
        >
          <template #empty>
            <div class="text-center py-8">
              <i class="pi pi-sitemap text-4xl text-gray-400 mb-2"></i>
              <h3 class="text-xl font-semibold text-gray-600">No Categories Found</h3>
              <p class="text-gray-500">Click the "New" button to add a category.</p>
            </div>
          </template>

          <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
          <Column field="name" header="Category Name" :sortable="true">
            <template #body="slotProps">
              <Tag :value="slotProps.data.name" class="text-lg" />
            </template>
          </Column>
          <Column field="description" header="Description" :sortable="true"></Column>
          <Column field="product_count" header="Products" :sortable="true" headerStyle="text-align: center" bodyStyle="text-align: center">
             <template #body="slotProps">
                <span class="font-medium">{{ slotProps.data.product_count }}</span>
            </template>
          </Column>
          <Column field="updated_at" header="Last Updated" :sortable="true">
            <template #body="slotProps">
              {{ formatDate(slotProps.data.updated_at) }}
            </template>
          </Column>
          <Column headerStyle="width: 10rem; text-align: center" bodyStyle="text-align: center">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2" severity="success" rounded @click="editCategory(slotProps.data)" v-tooltip.top="'Edit'" />
                <Button icon="pi pi-trash" severity="warning" rounded @click="confirmDeleteCategory(slotProps.data)" v-tooltip.top="'Delete'" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-model:visible="categoryDialog" :style="{width: '450px'}" header="Category Details" :modal="true" class="p-fluid">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="name" v-model.trim="category.name" required="true" autofocus :class="{'p-invalid': submitted && !category.name}" fluid />
                <label for="name">Name</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !category.name">Name is required.</small>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Textarea id="description" v-model="category.description" rows="3" cols="20" fluid />
                <label for="description">Description</label>
            </FloatLabel>
        </div>
      </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="saveCategory" />
        </template>
    </Dialog>

  </div>
</template>