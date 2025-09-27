import { ref, onUnmounted } from 'vue';

export function useWebSocket(url) {
  const data = ref(null);
  const ws = new WebSocket(url);

  ws.onmessage = (event) => {
    data.value = event.data;
  };

  ws.onopen = () => {
    console.log(`WebSocket connected to ${url}`);
  };

  ws.onerror = (error) => {
    console.error('WebSocket Error:', error);
  };

  const close = () => {
    ws.close();
  };

  onUnmounted(() => {
    close();
  });

  return {
    data,
    close,
  };
}
