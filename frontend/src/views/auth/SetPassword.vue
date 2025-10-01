<template>
  <div class="flex items-center justify-center min-h-screen bg-surface-100 dark:bg-surface-900">
    <Card class="w-full max-w-md shadow-lg">
      <template #title>
        <h2 class="text-2xl font-bold text-center">Set Your Password</h2>
      </template>
      <template #content>
        <form @submit.prevent="handleSubmit">
          <div class="field flex flex-col mb-4">
            <FloatLabel>
              <Password v-model="password" inputId="password" toggleMask :feedback="true" class="w-full" />
              <label for="password">New Password</label>
            </FloatLabel>
          </div>
          <div class="field flex flex-col mb-6">
            <FloatLabel>
              <Password v-model="confirmPassword" inputId="confirm_password" toggleMask :feedback="false" class="w-full" :class="{'p-invalid': passwordMismatch}" />
              <label for="confirm_password">Confirm New Password</label>
            </FloatLabel>
            <small v-if="passwordMismatch" class="p-error mt-1">Passwords do not match.</small>
          </div>
          <Button type="submit" label="Set Password" class="w-full" :loading="isLoading" />
        </form>
      </template>
    </Card>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useToast } from 'primevue/usetoast';
import Card from 'primevue/card';
import Button from 'primevue/button';
import Password from 'primevue/password';
import FloatLabel from 'primevue/floatlabel';
import { useAuthStore } from '../../store/auth';

const route = useRoute();
const router = useRouter();
const toast = useToast();
const authStore = useAuthStore();

const token = ref(null);
const password = ref('');
const confirmPassword = ref('');
const passwordMismatch = ref(false);
const isLoading = ref(false);

onMounted(() => {
  token.value = route.query.token;
  if (!token.value) {
    toast.add({ severity: 'error', summary: 'Error', detail: 'Invalid or missing token.', life: 5000 });
    router.push({ name: 'login' });
  }
});

const handleSubmit = async () => {
  if (password.value !== confirmPassword.value) {
    passwordMismatch.value = true;
    toast.add({ severity: 'warn', summary: 'Warning', detail: 'Passwords do not match.', life: 3000 });
    return;
  }
  passwordMismatch.value = false;
  isLoading.value = true;

  try {
    await authStore.setPassword(token.value, password.value);
    toast.add({ severity: 'success', summary: 'Success', detail: 'Password set successfully! You can now log in.', life: 5000 });
    router.push({ name: 'login' });
  } catch (error) {
    const errorMessage = error.response?.data?.message || 'Failed to set password.';
    toast.add({ severity: 'error', summary: 'Error', detail: errorMessage, life: 5000 });
  } finally {
    isLoading.value = false;
  }
};
</script>
