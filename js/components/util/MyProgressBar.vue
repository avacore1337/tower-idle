<!DOCTYPE html>
<template>
  <div class="progress" style="position: relative">
    <div id="myBar" class="my-bar" :style="{ width: value + '%' }"></div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  value: { type: Number, required: true },
})
</script>

<style scoped>
.progress {
  width: 100%;
  background-color: #ddd;
  height: 0.3rem;
}

.my-bar {
  position: relative;
  height: 0.3rem;
  background-color: #0d6efd;
}
</style>
