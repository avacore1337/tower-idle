<!DOCTYPE html>
<template>
  <vue-final-modal
    v-model="is_open"
    classes="modal-container"
    content-class="modal-content"
    :name="name"
    :esc-to-close="true"
  >
    <button class="modal__close" @click="is_open = false">X</button>
    <span class="modal__title">{{ title }}</span>
    <div class="modal__content">
      <slot />
    </div>
  </vue-final-modal>
</template>

<script setup lang="ts">
import { ref } from "vue"
let is_open = ref(false)
defineProps({
  name: {
    type: String,
    required: true,
  },
  title: {
    type: String,
    required: true,
  },
})
</script>

<style scoped></style>
