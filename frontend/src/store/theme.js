import { ref, watch, onMounted } from 'vue';
import { defineStore } from 'pinia';

export const useThemeStore = defineStore('theme', () => {
  // Initialize isDark from localStorage, default to false (light mode)
  const isDark = ref(JSON.parse(localStorage.getItem('isDark') || 'false'));

  // Function to apply the theme class
  const applyTheme = (isDarkValue) => {
    if (isDarkValue) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  };

  // Apply the theme immediately when the store is created
  applyTheme(isDark.value);

  // Watch for changes in isDark and update localStorage and the DOM
  watch(isDark, (newVal) => {
    localStorage.setItem('isDark', JSON.stringify(newVal));
    applyTheme(newVal);
  });

  // Action to toggle the theme
  function toggleTheme() {
    isDark.value = !isDark.value;
  }

  return {
    isDark,
    toggleTheme,
  };
});
