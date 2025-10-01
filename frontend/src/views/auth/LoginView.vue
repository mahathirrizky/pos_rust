<template>
  <div class="flex items-center justify-center min-h-screen bg-surface-100 dark:bg-surface-900">
    <div class="w-full px-8 py-6 mt-4 text-left bg-surface-0 dark:bg-surface-800 shadow-lg md:w-1/3">
      <h3 class="text-2xl font-bold text-center">Login to {{ formattedRole }} Account</h3>
      <form @submit.prevent="handleLogin" class="p-fluid">
        <div class="mt-4 mb-4">
          <FloatLabel variant="on">
            <InputText id="email" type="text" v-model="email" variant="filled" fluid />
            <label for="email">Email</label>
          </FloatLabel>
        </div>
        <div class="mt-4 mb-4">
          <FloatLabel variant="on">
            <Password id="password" v-model="password" :feedback="false" variant="filled" :toggleMask="true" fluid />
            <label for="password">Password</label>
          </FloatLabel>
        </div>
        <div class="flex items-baseline justify-between">
          <Button type="submit" :label="isLoading ? 'Logging in...' : 'Login'" :loading="isLoading" class="mt-4" />
          <a href="#" class="text-sm text-blue-600 hover:underline">Forgot password?</a>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useAuthStore } from '../../store/auth';
import InputText from 'primevue/inputtext';
import Password from 'primevue/password';
import Button from 'primevue/button';
import FloatLabel from 'primevue/floatlabel';

const router = useRouter();
const route = useRoute();
const authStore = useAuthStore();

const email = ref('owner@example.com'); // Default for easier testing
const password = ref('password123'); // Default for easier testing
const isLoading = ref(false);

const pageRole = computed(() => {
  const pathParts = route.path.split('/');
  if (pathParts.length > 1 && pathParts[1]) {
    const roleSlug = pathParts[1];
    return roleSlug.split('-').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join('');
  }
  return '';
});

const formattedRole = computed(() => {
  if (!pageRole.value) return '';
  return pageRole.value.replace(/([A-Z])/g, ' $1').trim();
});

const handleLogin = async () => {
  console.log('Login button clicked'); // Log 1: Button click event
  if (!email.value || !password.value) {
    alert('Email and password are required.');
    return;
  }

  isLoading.value = true;

  try {
    const credentials = {
        email: email.value,
        password: password.value,
        role: pageRole.value,
    };
    console.log('Sending credentials to store:', credentials); // Log 2: Credentials being sent

    await authStore.login(credentials);

    const dashboardPath = `/${pageRole.value.toLowerCase()}`;
    router.push(dashboardPath);

  } catch (error) {
    console.error('Login failed:', error);
    alert(`Login failed: ${error.message}`);
  } finally {
    isLoading.value = false;
  }
};
</script>

<style scoped>
/* Your styles here */
</style>