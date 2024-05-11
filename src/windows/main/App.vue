<script setup lang="ts">
import { showWindow } from '@/lib/commands';
import { setGlobalSensors } from '@/lib/sensors';

setGlobalSensors();

onMounted(() => {
  flushPromises().then(showWindow).catch(handleError);
});
</script>

<template>
  <main>
    <router-view #default="{ Component }">
      <template v-if="Component">
        <component :is="Component" />
      </template>
    </router-view>
  </main>
</template>
