<template>
  <div class="flex items-center justify-center min-h-screen bg-surface-100 dark:bg-surface-900">
    <Card class="w-full max-w-md shadow-lg">
      <template #title>
        <h2 class="text-2xl font-bold text-center">Forgot Password</h2>
      </template>
      <template #subtitle>
        <p class="text-center text-surface-600 dark:text-surface-400">Enter your email address and we will send you a link to reset your password.</p>
      </template>
      <template #content>
        <form @submit.prevent="handleSubmit">
          <div class="field flex flex-col mb-4">
            <FloatLabel>
              <InputText id="email" v-model="email" type="email" class="w-full" />
              <label for="email">Email Address</label>
            </FloatLabel>
          </div>
          <Button type="submit" label="Send Reset Link" class="w-full" :loading="isLoading" />
        </form>
        <div class="mt-4 text-center">
          <router-link to="/admin/login" class="text-primary-500 hover:underline">Back to Login</router-link>
        </div>
      </template>
    </Card>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useToast } from 'primevue/usetoast';
import Card from 'primevue/card';
import Button from 'primevue/button';
import InputText from 'primevue/inputtext';
import FloatLabel from 'primevue/floatlabel';
import { useAuthStore } from '../../store/auth';

const toast = useToast();
const authStore = useAuthStore();

const email = ref('');
const isLoading = ref(false);

const handleSubmit = async () => {
  if (!email.value) {
    toast.add({ severity: 'warn', summary: 'Warning', detail: 'Please enter your email address.', life: 3000 });
    return;
  }
  isLoading.value = true;

  try {
    await authStore.forgotPassword(email.value);
    toast.add({ 
      severity: 'success', 
      summary: 'Check Your Email', 
      detail: 'If an account with that email exists, a password reset link has been sent.', 
      life: 5000 
    });
  } catch (error) {
    // Even on error, we show a generic success message to prevent email enumeration
    toast.add({ 
      severity: 'success', 
      summary: 'Check Your Email', 
      detail: 'If an account with that email exists, a password reset link has been sent.', 
      life: 5000 
    });
  } finally {
    isLoading.value = false;
    email.value = '';
  }
};
</script>
