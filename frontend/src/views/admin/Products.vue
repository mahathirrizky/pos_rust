<script setup>
import { ref, onMounted, computed, defineAsyncComponent } from 'vue';
import { useConfirm } from "primevue/useconfirm";
import { useToast } from "primevue/usetoast";
import { z } from 'zod';
import { zodResolver } from '@primevue/forms/resolvers/zod';
import { useProductStore } from '../../store/product';

// PrimeVue Components
import { Form } from '@primevue/forms';
import Message from 'primevue/message';
const DataView = defineAsyncComponent(() => import('primevue/dataview'));
const Card = defineAsyncComponent(() => import('primevue/card'));
const Toolbar = defineAsyncComponent(() => import('primevue/toolbar'));
const Button = defineAsyncComponent(() => import('primevue/button'));
const Dialog = defineAsyncComponent(() => import('primevue/dialog'));
const InputText = defineAsyncComponent(() => import('primevue/inputtext'));
const Textarea = defineAsyncComponent(() => import('primevue/textarea'));
const InputNumber = defineAsyncComponent(() => import('primevue/inputnumber'));
const Select = defineAsyncComponent(() => import('primevue/select'));
const FloatLabel = defineAsyncComponent(() => import('primevue/floatlabel'));
const FileUpload = defineAsyncComponent(() => import('primevue/fileupload'));
const DatePicker = defineAsyncComponent(() => import('primevue/datepicker'));

const confirm = useConfirm();
const toast = useToast();
const productStore = useProductStore();

const productDialog = ref(false);
const editProductDialog = ref(false);
const product = ref({});
const searchQuery = ref('');
const originalPhotoUrl = ref(null);
const newlyUploadedPhotoUrl = ref(null);

// Zod Schema and Resolver
const productSchema = z.object({
    id: z.number().optional(),
    name: z.string().min(1, 'Name is required.'),
    description: z.string().optional(),
    price: z.number().min(0.01, 'Price must be greater than 0.'),
    sku: z.string().min(1, 'SKU is required.'),
    category_id: z.number({ required_error: 'Category is required.' }),
    supplier_id: z.number({ required_error: 'Supplier is required.' }),
    photo_url: z.string().optional().nullable(),
    expires_at: z.date().optional().nullable(),
});

const resolver = ref(zodResolver(productSchema));

const filteredProducts = computed(() => {
  return productStore.products.filter(prod =>
    prod.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    prod.sku.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

onMounted(() => {
  productStore.fetchProducts();
  productStore.fetchCategories();
  productStore.fetchSuppliers();
});

const openNew = () => {
  product.value = {
    name: '',
    description: '',
    price: 0,
    sku: '',
    category_id: null,
    supplier_id: null,
    photo_url: null
  };
  productDialog.value = true;
};

const hideDialog = () => {
  productDialog.value = false;
};

const hideEditDialog = () => {
  editProductDialog.value = false;
};

const onFormSubmit = async ({ values }) => {
  try {
    const isUpdate = !!values.id;
    await productStore.saveProduct(values);
    productStore.fetchProducts(); // Refresh list
    toast.add({ 
        severity: 'success', 
        summary: 'Success', 
        detail: `Product ${isUpdate ? 'updated' : 'created'} successfully.`,
        life: 3000 
    });
    
    if (isUpdate) {
        // Clean up old photo if a new one was uploaded
        if (newlyUploadedPhotoUrl.value && originalPhotoUrl.value && newlyUploadedPhotoUrl.value !== originalPhotoUrl.value) {
            await productStore.removePhoto(originalPhotoUrl.value);
        }
        editProductDialog.value = false;
    } else {
        productDialog.value = false;
    }

    // Reset photo state
    originalPhotoUrl.value = null;
    newlyUploadedPhotoUrl.value = null;

  } catch (err) {
    toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
  }
};

const editProduct = (prod) => {
  product.value = { ...prod };
  originalPhotoUrl.value = prod.photo_url;
  newlyUploadedPhotoUrl.value = null;
  editProductDialog.value = true;
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

const formatCurrency = (value) => {
    if (typeof value !== 'number') return value;
    return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR' }).format(value);
};

const onUploadSuccess = (event, form) => {
  const response = JSON.parse(event.xhr.response);
  form.setFieldValue('photo_url', response.url);
  newlyUploadedPhotoUrl.value = response.url; // Track the new URL
  toast.add({ severity: "info", summary: "Success", detail: "File Uploaded", life: 3000 });
};

</script>

<template>
  <div>
    <Card>
      <template #title>
        <div class="flex justify-between items-center">
          <span class="text-2xl font-semibold">Product Management (Admin)</span>
        </div>
      </template>
      <template #content>
        <Toolbar class="mb-4">
          <template #start>
            <Button label="New" icon="pi pi-plus" class="mr-2" @click="openNew" />
          </template>
          <template #end>
            <div class="flex items-center">
              <InputText placeholder="Search..." v-model="searchQuery" class="w-full" />
            </div>
          </template>
        </Toolbar>

        <DataView :value="filteredProducts" layout="grid" :paginator="true" :rows="9">
          <template #grid="slotProps">
            <div class="grid grid-nogutter">
              <div v-for="item in slotProps.items" :key="item.id" class="col-12 md:col-4">
              <Card class="m-2">
                <template #header>
                  <img :src="item.photo_url || '/placeholder.svg'" :alt="item.name" class="w-full h-48 object-cover" />
                </template>
                <template #title>
                  <div class="flex justify-between items-start">
                    <span class="font-bold text-lg">{{ item.name }}</span>
                    <div class="flex">
                      <Button icon="pi pi-pencil" class="p-button-rounded p-button-success p-button-text" @click="editProduct(item)" />
                      <Button icon="pi pi-trash" class="p-button-rounded p-button-danger p-button-text" @click="confirmDeleteProduct(item)" />
                    </div>
                  </div>
                </template>
                <template #subtitle>
                  <span class="text-sm text-surface-500 dark:text-surface-400">SKU: {{ item.sku }}</span>
                </template>
                <template #content>
                  <div class="flex flex-col gap-2">
                    <div class="text-2xl font-bold">{{ formatCurrency(item.price) }}</div>
                    <div>Category: <span class="font-semibold">{{ item.category_name }}</span></div>
                    <div>Supplier: <span class="font-semibold">{{ item.supplier_name }}</span></div>
                  </div>
                </template>
              </Card>
            </div>
            </div>
          </template>
        </DataView>
      </template>
    </Card>

    <Dialog v-model:visible="productDialog" :style="{width: '450px'}" header="New Product" :modal="true" class="p-fluid">
      <Form v-slot="$form" :initialValues="product" :resolver="resolver" @submit="onFormSubmit" class="flex flex-col gap-4">
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText fluid name="name" id="name" type="text" :class="{ 'p-invalid': $form.name?.invalid }" />
                <label for="name">Name</label>
            </FloatLabel>
            <Message v-if="$form.name?.invalid" severity="error" size="small" variant="simple">{{ $form.name.error.message }}</Message>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Textarea fluid name="description" id="description" rows="3" />
                <label for="description">Description</label>
            </FloatLabel>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputNumber fluid name="price" id="price" mode="currency" currency="IDR" locale="id-ID" :class="{ 'p-invalid': $form.price?.invalid }" />
                <label for="price">Price</label>
            </FloatLabel>
            <Message v-if="$form.price?.invalid" severity="error" size="small" variant="simple">{{ $form.price.error.message }}</Message>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <InputText fluid name="sku" id="sku" type="text" :class="{ 'p-invalid': $form.sku?.invalid }" />
                <label for="sku">SKU</label>
            </FloatLabel>
            <Message v-if="$form.sku?.invalid" severity="error" size="small" variant="simple">{{ $form.sku.error.message }}</Message>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Select name="category_id" id="category_id" :options="productStore.categories" optionLabel="label" optionValue="value" :class="{ 'p-invalid': $form.category_id?.invalid }" fluid />
                <label for="category_id">Category</label>
            </FloatLabel>
            <Message v-if="$form.category_id?.invalid" severity="error" size="small" variant="simple">{{ $form.category_id.error.message }}</Message>
        </div>
        <div class="field col-12 p-2">
            <FloatLabel variant="on">
                <Select name="supplier_id" id="supplier_id" :options="productStore.suppliers" optionLabel="label" optionValue="value" :class="{ 'p-invalid': $form.supplier_id?.invalid }" fluid />
                <label for="supplier_id">Supplier</label>
            </FloatLabel>
            <Message v-if="$form.supplier_id?.invalid" severity="error" size="small" variant="simple">{{ $form.supplier_id.error.message }}</Message>
        </div>
        <div class="field col-12 p-2">
            <FileUpload name="photo_url" url="/api/upload/product" @upload="onUploadSuccess($event, $form)" :multiple="false" accept="image/*" :maxFileSize="1000000">
                <template #empty><p>Drag and drop files to here to upload.</p></template>
            </FileUpload>
        </div>
        <div class="flex gap-2 justify-end">
            <Button label="Cancel" icon="pi pi-times" text @click="hideDialog"/>
            <Button label="Save" icon="pi pi-check" text type="submit" />
        </div>
      </Form>
    </Dialog>

    <Dialog v-model:visible="editProductDialog" :style="{width: '450px'}" header="Edit Product" :modal="true" class="p-fluid">
        <Form v-slot="$form" :initialValues="product" :resolver="resolver" @submit="onFormSubmit" class="flex flex-col gap-4">
            <div v-if="$form.values.photo_url" class="mb-4">
                <img :src="$form.values.photo_url" :alt="$form.values.name" class="w-full h-auto rounded max-w-xs" />
            </div>
            <div class="field">
                <FloatLabel>
                    <InputText name="name" id="edit-name" type="text" :class="{ 'p-invalid': $form.name?.invalid }" />
                    <label for="edit-name">Name</label>
                </FloatLabel>
                <Message v-if="$form.name?.invalid" severity="error" size="small" variant="simple">{{ $form.name.error.message }}</Message>
            </div>
            <div class="field">
                <FloatLabel>
                    <Textarea name="description" id="edit-description" rows="3" />
                    <label for="edit-description">Description</label>
                </FloatLabel>
            </div>
            <div class="field">
                <FloatLabel>
                    <InputNumber name="price" id="edit-price" mode="currency" currency="IDR" locale="id-ID" :class="{ 'p-invalid': $form.price?.invalid }" />
                    <label for="edit-price">Price</label>
                </FloatLabel>
                <Message v-if="$form.price?.invalid" severity="error" size="small" variant="simple">{{ $form.price.error.message }}</Message>
            </div>
            <div class="field">
                <FloatLabel>
                    <InputText name="sku" id="edit-sku" type="text" :class="{ 'p-invalid': $form.sku?.invalid }" />
                    <label for="edit-sku">SKU</label>
                </FloatLabel>
                <Message v-if="$form.sku?.invalid" severity="error" size="small" variant="simple">{{ $form.sku.error.message }}</Message>
            </div>
            <div class="field">
                <FloatLabel>
                    <Select name="category_id" id="edit-category_id" :options="productStore.categories" optionLabel="label" optionValue="value" :class="{ 'p-invalid': $form.category_id?.invalid }"></Select>
                    <label for="edit-category_id">Category</label>
                </FloatLabel>
                <Message v-if="$form.category_id?.invalid" severity="error" size="small" variant="simple">{{ $form.category_id.error.message }}</Message>
            </div>
            <div class="field">
                <FloatLabel>
                    <Select name="supplier_id" id="edit-supplier_id" :options="productStore.suppliers" optionLabel="label" optionValue="value" :class="{ 'p-invalid': $form.supplier_id?.invalid }"></Select>
                    <label for="edit-supplier_id">Supplier</label>
                </FloatLabel>
                <Message v-if="$form.supplier_id?.invalid" severity="error" size="small" variant="simple">{{ $form.supplier_id.error.message }}</Message>
            </div>
            <div class="field">
                <FloatLabel>
                    <DatePicker name="expires_at" id="edit-expires_at" dateFormat="yy-mm-dd" :class="{ 'p-invalid': $form.expires_at?.invalid }" />
                    <label for="edit-expires_at">Expires At</label>
                </FloatLabel>
                <Message v-if="$form.expires_at?.invalid" severity="error" size="small" variant="simple">{{ $form.expires_at.error.message }}</Message>
            </div>
            <div class="field">
                <FloatLabel>
                    <DatePicker name="expires_at" id="expires_at" dateFormat="yy-mm-dd" :class="{ 'p-invalid': $form.expires_at?.invalid }" />
                    <label for="expires_at">Expires At</label>
                </FloatLabel>
                <Message v-if="$form.expires_at?.invalid" severity="error" size="small" variant="simple">{{ $form.expires_at.error.message }}</Message>
            </div>
            <div class="field">
                <label>Change Photo</label>
                <FileUpload name="photo_url" url="/api/upload/product" @upload="onUploadSuccess($event, $form)" :multiple="false" accept="image/*" :maxFileSize="1000000">
                    <template #empty><p>Drag and drop files to here to upload.</p></template>
                </FileUpload>
            </div>
            <div class="flex gap-2 justify-end">
                <Button label="Cancel" icon="pi pi-times" text @click="hideEditDialog"/>
                <Button label="Save" icon="pi pi-check" text type="submit" />
            </div>
      </Form>
    </Dialog>

  </div>
</template>
