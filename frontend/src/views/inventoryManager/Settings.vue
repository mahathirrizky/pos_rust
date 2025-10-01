<script setup>
import { useThemeStore } from '../../store/theme';
import { storeToRefs } from 'pinia';
import ToggleSwitch from 'primevue/toggleswitch';

const themeStore = useThemeStore();
const { isDark } = storeToRefs(themeStore);
</script>

<template>
  <div class="card">
    <h1 class="text-2xl font-bold mb-6">Inventory Manager Settings</h1>

    <div class="field">
      <label for="theme-switcher" class="font-bold mb-2 block">Dark Mode</label>
      <div class="flex items-center">
        <ToggleSwitch v-model="isDark" inputId="theme-switcher" />
        <span class="ml-2">{{ isDark ? 'On' : 'Off' }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.card {
  background: var(--surface-card);
  padding: 2rem;
  border-radius: 10px;
  margin-bottom: 1rem;
}

.field {
  margin-bottom: 1.5rem;
}

.flex {
  display: flex;
}

.items-center {
  align-items: center;
}

.ml-2 {
  margin-left: 0.5rem;
}
</style>
