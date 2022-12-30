<!DOCTYPE html>
<template>
  <vue-final-modal
    v-model="is_open"
    classes="modal-container"
    content-class="modal-content"
    name="general"
    :esc-to-close="true"
  >
    <button class="modal__close" @click="is_open = false">X</button>
    <span class="modal__title">General</span>
    <div class="modal__content">
      <MyToggle :value="settings.autosave" :click="() => (settings.autosave = !settings.autosave)">
        Autosave
      </MyToggle>
      <MyToggle
        :value="settings.use_saved_ticks"
        :click="() => (settings.use_saved_ticks = !settings.use_saved_ticks)"
      >
        Use saved time
      </MyToggle>
    </div>
  </vue-final-modal>
</template>

<script setup lang="ts">
import { ref, watch, computed } from "vue"
import { useStore } from "@store"
import MyToggle from "@c/util/MyToggle.vue"

let store = useStore()
let wasm = computed(() => store.state.wasm)
let settings = ref(store.state.user_settings)

watch(settings, save_settings, { deep: true })

function save_settings() {
  wasm.value.set_user_settings(settings.value)
}

let is_open = ref(false)
</script>

<style scoped></style>
