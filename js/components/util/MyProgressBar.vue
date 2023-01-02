<!DOCTYPE html>
<template>
  <div class="progress" style="position: relative">
    <div id="myBar" :class="'my-bar ' + color" :style="{ width: value + '%' }"></div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  value: { type: Number, required: true },
  color: { type: String, default: "progress-base" },
})
</script>

<style scoped>
.progress {
  width: 100%;
  background-color: var(--tower-progress-bar-other);
  height: 0.3rem;
}

.my-bar {
  position: relative;
  height: 0.3rem;
}
.progress-base {
  background-color: var(--tower-progress-bar);
}
.progress-other {
  background-color: var(--tower-other-color);
}
.progress-other-other {
  background-color: var(--tower-other-other-color);
}
</style>
