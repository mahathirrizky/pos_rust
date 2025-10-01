<script setup>
import { ref, onMounted, defineAsyncComponent } from 'vue';
import { storeToRefs } from 'pinia';
import { useConfirm } from "primevue/useconfirm";
import { z } from 'zod';
import { zodResolver } from '@primevue/forms/resolvers/zod';
import { Form } from '@primevue/forms';
import Message from 'primevue/message';
import { useToast } from "primevue/usetoast";
import { useProductStore } from '../../store/product';
import { useCategoryStore } from '../../store/category';

// Async Components
const DataView = defineAsyncComponent(() => import('primevue/dataview'));
const Tag = defineAsyncComponent(() => import('primevue/tag'));
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
const categoryStore = useCategoryStore();

const { products, totalRecords, loading } = storeToRefs(productStore);
const { categories } = storeToRefs(categoryStore);

const selectedProducts = ref([]);
const productDialog = ref(false);
const product = ref({});

const productSchema = z.object({
    id: z.number().optional(),
    name: z.string().min(1, 'Name is required.'),
    description: z.string().optional(),
    price: z.number().min(0.01, 'Price must be greater than 0.').nullable(),
    sku: z.string().min(1, 'SKU is required.'),
    category_id: z.number({ required_error: 'Category is required.' }).nullable(),
    supplier_id: z.number({ required_error: 'Supplier is required.' }).nullable(),
    photo_url: z.string().optional().nullable(),
    expires_at: z.date().optional().nullable(),
    stock: z.number().optional(),
    rating: z.number().optional().nullable(),
});

const resolver = ref(zodResolver(productSchema));

const layout = ref('grid');
const first = ref(0);
const rows = ref(6);
const sortOrder = ref(1);
const sortField = ref('price');


onMounted(() => {
    productStore.fetchProducts({
        page: 1,
        per_page: rows.value,
        sort_by: sortField.value,
        sort_order: sortOrder.value === 1 ? 'asc' : 'desc'
    });
    productStore.fetchCategories();
    productStore.fetchSuppliers();
});

const onPage = (event) => {
    first.value = event.first;
    rows.value = event.rows;
    productStore.fetchProducts({
        page: event.page + 1,
        per_page: event.rows,
        sort_by: sortField.value,
        sort_order: sortOrder.value === 1 ? 'asc' : 'desc'
    });
};

const openNew = () => {
  product.value = { // Initialize with default values for the form
    name: '',
    description: '',
    price: null,
    sku: '',
    category_id: null,
    supplier_id: null,
    photo_url: null,
    stock: 0,
    rating: null,
  };
  productDialog.value = true;
};

const hideDialog = () => {
  productDialog.value = false;
};

const onFormSubmit = async ({ values }) => {
  try {
    await productStore.saveProduct(values);
    const isUpdate = !!values.id;
    toast.add({ severity: 'success', summary: 'Success', detail: `Product ${isUpdate ? 'updated' : 'created'} successfully.`, life: 3000 });
    productDialog.value = false;
    productStore.fetchProducts({
        page: 1,
        per_page: rows.value,
        sort_by: sortField.value,
        sort_order: sortOrder.value === 1 ? 'asc' : 'desc'
    });
  } catch (err) {
    toast.add({ severity: 'error', summary: 'Error', detail: err.message, life: 3000 });
  }
};

const editProduct = (prod) => {
  product.value = { ...prod }; // Set product.value to populate the form
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

const onUploadSuccess = (event, form) => {
  const response = JSON.parse(event.xhr.response);
  form.setFieldValue('photo_url', response.url);
  toast.add({ severity: 'info', summary: 'Success', detail: 'File Uploaded', life: 3000 });
};

const getImageUrl = (photo_url) => {
    return photo_url ? photo_url : 'https://via.placeholder.com/300';
};

const getInventoryStatus = (product) => {
    if (product.stock > 10) return 'INSTOCK';
    if (product.stock > 0) return 'LOWSTOCK';
    if (product.stock === 0) return 'OUTOFSTOCK';
    return 'UNKNOWN';
}

const getSeverity = (product) => {
    const status = getInventoryStatus(product);
    switch (status) {
        case 'INSTOCK':
            return 'success';
        case 'LOWSTOCK':
            return 'warning';
        case 'OUTOFSTOCK':
            return 'danger';
        default:
            return null;
    }
};

const getCategoryName = (categoryId) => {
    const category = categories.value.find(c => c.id === categoryId);
    return category ? category.name : 'Uncategorized';
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

        <DataView :value="products" :layout="layout" :paginator="true" :rows="rows" :sortOrder="sortOrder" :sortField="sortField" @page="onPage">
            <template #header>
                <div class="flex justify-end">
                    <DataViewLayoutOptions v-model="layout" />
                </div>
            </template>

            <template #list="slotProps">
                <div class="grid grid-nogutter">
                    <div v-for="(item, index) in slotProps.items" :key="index" class="col-12">
                        <div class="flex flex-column xl:flex-row xl:align-items-start p-4 gap-4" :class="{'border-top-1 surface-border': index !== 0}">
                            <img class="w-9 sm:w-16rem xl:w-10rem shadow-2 block xl:block mx-auto border-round" :src="getImageUrl(item.photo_url)" :alt="item.name" />
                            <div class="flex flex-column sm:flex-row justify-content-between align-items-center xl:align-items-start flex-1 gap-4">
                                <div class="flex flex-column align-items-center sm:align-items-start gap-3">
                                    <div class="text-2xl font-bold text-900">{{ item.name }}</div>
                                    <span class="font-medium text-secondary text-sm">{{ getCategoryName(item.category_id) }}</span>
                                    <p class="text-sm text-600 mt-1">{{ item.description }}</p>
                                    <div class="flex align-items-center gap-3">
                                        <span class="flex align-items-center gap-2">
                                            <i class="pi pi-tag"></i>
                                            <span class="font-semibold">{{ getCategoryName(item.category_id) }}</span>
                                        </span>
                                        <Tag :value="getInventoryStatus(item)" :severity="getSeverity(item)"></Tag>
                                    </div>
                                </div>
                                <div class="flex sm:flex-column align-items-center sm:align-items-end gap-3 sm:gap-2">
                                    <span class="text-2xl font-semibold">${{ item.price }}</span>
                                    <div class="flex gap-2 mt-2">
                                        <Button icon="pi pi-pencil" class="p-button-rounded p-button-success" @click="editProduct(item)" />
                                        <Button icon="pi pi-trash" class="p-button-rounded p-button-danger" @click="confirmDeleteProduct(item)" />
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </template>

            <template #grid="slotProps">
                <div class="grid grid-nogutter">
                    <div v-for="(item, index) in slotProps.items" :key="index" class="col-12 sm:col-6 md:col-4 xl:col-6 p-2">
                        <div class="p-4 border-1 surface-border surface-card border-round flex flex-column">
                            <div class="surface-50 flex justify-content-center border-round p-3">
                                <div class="relative mx-auto">
                                    <img class="border-round w-full" :src="getImageUrl(item.photo_url)" :alt="item.name" style="max-width: 300px"/>
                                    <Tag :value="getInventoryStatus(item)" :severity="getSeverity(item)" class="absolute" style="left: 4px; top: 4px"></Tag>
                                </div>
                            </div>
                            <div class="pt-4">
                                <div class="flex flex-row justify-content-between align-items-start gap-2">
                                    <div>
                                        <span class="font-medium text-secondary text-sm">{{ getCategoryName(item.category_id) }}</span>
                                        <div class="text-lg font-medium text-900 mt-1">{{ item.name }}</div>
                                        <p class="text-sm text-600 mt-1">{{ item.description }}</p>
                                    </div>
                                    <div class="surface-100 p-1" style="border-radius: 30px">
                                        <div class="surface-0 flex align-items-center gap-2 justify-content-center py-1 px-2" style="border-radius: 30px; box-shadow: 0px 1px 2px 0px rgba(0, 0, 0, 0.04), 0px 1px 2px 0px rgba(0, 0, 0, 0.06)">
                                            <span class="text-900 font-medium text-sm">{{ item.rating }}</span>
                                            <i class="pi pi-star-fill text-yellow-500"></i>
                                        </div>
                                    </div>
                                </div>
                                <div class="flex flex-column gap-4 mt-4">
                                    <span class="text-2xl font-semibold text-900">${{ item.price }}</span>
                                    <div class="flex gap-2">
                                        <Button icon="pi pi-pencil" class="p-button-rounded p-button-success" @click="editProduct(item)" />
                                        <Button icon="pi pi-trash" class="p-button-rounded p-button-danger" @click="confirmDeleteProduct(item)" />
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </template>
        </DataView>
      </template>
    </Card>

    <Dialog v-model:visible="productDialog" :style="{width: '450px'}" :header="product.id ? 'Edit Product' : 'New Product'" :modal="true" class="p-fluid">
        <Form v-slot="$form" :initialValues="product" :resolver="resolver" @submit="onFormSubmit" class="flex flex-col gap-4">
            <div class="field">
                <FloatLabel variant="on">
                    <InputText name="name" id="name" type="text" fluid :class="{ 'p-invalid': $form.name?.invalid }" />
                    <label for="name">Name</label>
                </FloatLabel>
                <Message v-if="$form.name?.invalid" severity="error" size="small" variant="simple">{{ $form.name.error.message }}</Message>
            </div>
            <div class="field">
                <FloatLabel variant="on">
                    <Textarea name="description" id="description" rows="3" fluid />
                    <label for="description">Description</label>
                </FloatLabel>
            </div>
            <div class="field">
                <FloatLabel variant="on">
                    <InputNumber name="price" id="price" mode="decimal" :minFractionDigits="2" :maxFractionDigits="2" fluid :class="{ 'p-invalid': $form.price?.invalid }" />
                    <label for="price">Price</label>
                </FloatLabel>
                <Message v-if="$form.price?.invalid" severity="error" size="small" variant="simple">{{ $form.price.error.message }}</Message>
            </div>
            <div class="field">
                <FloatLabel variant="on">
                    <InputText name="sku" id="sku" type="text" fluid :class="{ 'p-invalid': $form.sku?.invalid }" />
                    <label for="sku">SKU</label>
                </FloatLabel>
                <Message v-if="$form.sku?.invalid" severity="error" size="small" variant="simple">{{ $form.sku.error.message }}</Message>
            </div>
            <div class="field">
                <FloatLabel variant="on">
                    <Select name="category_id" id="category_id" :options="productStore.categories" optionLabel="label" optionValue="value" fluid :class="{ 'p-invalid': $form.category_id?.invalid }"></Select>
                    <label for="category_id">Category</label>
                </FloatLabel>
                <Message v-if="$form.category_id?.invalid" severity="error" size="small" variant="simple">{{ $form.category_id.error.message }}</Message>
            </div>
            <div class="field">
                <FloatLabel variant="on">
                    <Select name="supplier_id" id="supplier_id" :options="productStore.suppliers" optionLabel="label" optionValue="value" fluid :class="{ 'p-invalid': $form.supplier_id?.invalid }"></Select>
                    <label for="supplier_id">Supplier</label>
                </FloatLabel>
                <Message v-if="$form.supplier_id?.invalid" severity="error" size="small" variant="simple">{{ $form.supplier_id.error.message }}</Message>
            </div>
            <div class="field">
                <FloatLabel variant="on">
                    <DatePicker name="expires_at" id="expires_at" dateFormat="yy-mm-dd" :class="{ 'p-invalid': $form.expires_at?.invalid }" />
                    <label for="expires_at">Expires At</label>
                </FloatLabel>
                <Message v-if="$form.expires_at?.invalid" severity="error" size="small" variant="simple">{{ $form.expires_at.error.message }}</Message>
            </div>
            <div class="field">
                 <label>Product Photo</label>
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

  </div>
</template>
