<script setup>
import { ref, onMounted } from 'vue';
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Card from 'primevue/card';
import Toolbar from 'primevue/toolbar';
import Button from 'primevue/button';
import Dialog from 'primevue/dialog';
import InputText from 'primevue/inputtext';
import Textarea from 'primevue/textarea';
import Tag from 'primevue/tag';
import ConfirmDialog from 'primevue/confirmdialog';
import FloatLabel from 'primevue/floatlabel';
import { useCategoryStore } from '../../store/category';

const confirm = useConfirm();
const toast = useToast();
const categoryStore = useCategoryStore();

const selectedCategories = ref([]);
const categoryDialog = ref(false);
const submitted = ref(false);
const category = ref({});

onMounted(() => {
  categoryStore.fetchCategories();
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

const saveCategory = async () => {
  submitted.value = true;
  if (category.value.name) {
    try {
      await categoryStore.saveCategory(category.value);
      categoryDialog.value = false;
      category.value = {};
      toast.add({ severity: 'success', summary: 'Success', detail: 'Category saved successfully', life: 3000 });
    } catch (error) {
      toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to save category', life: 3000 });
    }
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
        accept: async () => {
            try {
                await categoryStore.deleteCategory(cat.id);
                toast.add({ severity: 'success', summary: 'Success', detail: 'Category deleted successfully', life: 3000 });
            } catch (error) {
                toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to delete category', life: 3000 });
            }
        }
    });
};

const deleteSelectedCategories = () => {
    confirm.require({
        message: `Are you sure you want to delete the ${selectedCategories.value.length} selected categories?`,
        header: 'Confirmation',
        icon: 'pi pi-exclamation-triangle',
        accept: async () => {
            try {
                await Promise.all(selectedCategories.value.map(cat => categoryStore.deleteCategory(cat.id)));
                selectedCategories.value = [];
                toast.add({ severity: 'success', summary: 'Success', detail: 'Selected categories deleted successfully', life: 3000 });
            } catch (error) {
                toast.add({ severity: 'error', summary: 'Error', detail: error.message || 'Failed to delete selected categories', life: 3000 });
            }
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
    
    <ConfirmDialog />
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

        <div v-if="categoryStore.isLoading" class="text-center py-8">
          <i class="pi pi-spin pi-spinner text-4xl text-blue-500"></i>
          <p class="text-lg mt-2">Loading Categories...</p>
        </div>
        <div v-else-if="categoryStore.error" class="text-center py-8 text-red-500">
          <p>Error: {{ categoryStore.error.message }}</p>
          <Button label="Retry" @click="categoryStore.fetchCategories()" class="mt-4" />
        </div>
        <DataTable 
          v-else
          v-model:selection="selectedCategories" 
          :value="categoryStore.categories" 
          responsiveLayout="scroll"
          paginator :rows="10"
          :rowsPerPageOptions="[10, 20, 50]"
        >
          <template #empty>
            <div class="text-center py-8">
              <i class="pi pi-sitemap text-4xl text-surface-400 dark:text-surface-500 mb-2"></i>
              <h3 class="text-xl font-semibold text-surface-600 dark:text-surface-300">No Categories Found</h3>
              <p class="text-surface-500 dark:text-surface-400">Click the "New" button to add a category.</p>
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
                <Button icon="pi pi-trash" severity="danger" rounded @click="confirmDeleteCategory(slotProps.data)" v-tooltip.top="'Delete'" />
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