import { defineStore } from 'pinia';

export const useStore = defineStore('store', () => {
  const hello = ref('Hello, world!');

  return {
    hello
  };
});
