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
import InputNumber from 'primevue/inputnumber';
import Dropdown from 'primevue/dropdown';
import FloatLabel from 'primevue/floatlabel';
import { useProductStore } from '../../store/product';

const confirm = useConfirm();
const toast = useToast();
const productStore = useProductStore();

const selectedProducts = ref([]);
const productDialog = ref(false);
const submitted = ref(false);
const product = ref({});

onMounted(() => {
  productStore.fetchProducts();
  productStore.fetchCategories();
  productStore.fetchSuppliers();
});

const openNew = () => {
  product.value = {};
  submitted.value = false;
  productDialog.value = true;
};

const hideDialog = () => {
  productDialog.value = false;
  submitted.value = false;
};

const saveProduct = async () => {
  submitted.value = true;
  if (!product.value.name || !product.value.price || !product.value.sku) {
    return;
  }

  try {
    await productStore.saveProduct(product.value);
    const isUpdate = !!product.value.id;
    toast.add({ severity: 'success', summary: 'Success', detail: `Product ${isUpdate ? 'updated' : 'created'} successfully.`, life: 3000 });
    productDialog.value = false;
  } catch (err) {
    toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
  }
};

const editProduct = (prod) => {
  product.value = { ...prod };
  productDialog.value = true;
};

const confirmDeleteProduct = (prod) => {
    confirm.require({
        message: `Are you sure you want to delete ${prod.name}?`,
        header: 'Delete Confirmation',
        icon: 'pi pi-info-circle',
        acceptClass: 'p-button-danger',
        accept: async () => {
            try {
                await productStore.deleteProduct(prod.id);
                toast.add({ severity: 'success', summary: 'Confirmed', detail: 'Product deleted', life: 3000 });
            } catch (err) {
                toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
            }
        },
    });
};

const deleteSelectedProducts = () => {
    confirm.require({
        message: 'Are you sure you want to delete the selected products?',
        header: 'Delete Confirmation',
        icon: 'pi pi-info-circle',
        acceptClass: 'p-button-danger',
        accept: async () => {
            try {
                await Promise.all(selectedProducts.value.map(prod => productStore.deleteProduct(prod.id)));
                toast.add({ severity: 'success', summary: 'Confirmed', detail: 'Selected products deleted', life: 3000 });
                selectedProducts.value = [];
            } catch (err) {
                toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to delete selected products.', life: 3000 });
            }
        },
    });
};

const formatCurrency = (value) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(value);
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Product Management</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New" icon="pi pi-plus" class="mr-2" @click="openNew" />
            <Button label="Delete" icon="pi pi-trash" severity="danger" @click="deleteSelectedProducts" :disabled="!selectedProducts || !selectedProducts.length" />
          </template>
        </Toolbar>

        <DataTable v-model:selection="selectedProducts" :value="productStore.products" responsiveLayout="scroll">
          <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
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
          <Column headerStyle="width: 10rem">
             <template #body="slotProps">
                <Button icon="pi pi-pencil" class="mr-2 p-button-rounded p-button-success" @click="editProduct(slotProps.data)" />
                <Button icon="pi pi-trash" class="p-button-rounded p-button-danger" @click="confirmDeleteProduct(slotProps.data)" />
            </template>
          </Column>
        </DataTable>
      </template>
    </Card>

    <Dialog v-model:visible="productDialog" :style="{width: '450px'}" header="Product Details" :modal="true" class="p-fluid">
      <div class="p-grid p-fluid grid-nogutter">
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="name" v-model.trim="product.name" required="true" autofocus :class="{'p-invalid': submitted && !product.name}" variant="filled" fluid />
                <label for="name">Name</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !product.name">Name is required.</small>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Textarea id="description" v-model="product.description" rows="3" cols="20" variant="filled" fluid />
                <label for="description">Description</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputNumber id="price" v-model="product.price" mode="decimal" :minFractionDigits="2" :maxFractionDigits="2" required="true" :class="{'p-invalid': submitted && !product.price}" variant="filled" fluid />
                <label for="price">Price</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !product.price">Price is required.</small>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText id="sku" v-model.trim="product.sku" required="true" :class="{'p-invalid': submitted && !product.sku}" variant="filled" fluid />
                <label for="sku">SKU</label>
            </FloatLabel>
            <small class="p-error" v-if="submitted && !product.sku">SKU is required.</small>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Dropdown id="category_id" v-model="product.category_id" :options="productStore.categories" optionLabel="label" optionValue="value" placeholder="Select a Category" fluid variant="filled"></Dropdown>
                <label for="category_id">Category</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Dropdown id="supplier_id" v-model="product.supplier_id" :options="productStore.suppliers" optionLabel="label" optionValue="value" placeholder="Select a Supplier" fluid variant="filled"></Dropdown>
                <label for="supplier_id">Supplier</label>
            </FloatLabel>
        </div>
      </div>
        <template #footer>
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text @click="saveProduct" />
        </template>
    </Dialog>

  </div>
</template>