<!DOCTYPE html>
<template>
  <MyModal name="general" title="General">
    <MyToggle :value="settings.autosave" :click="() => (settings.autosave = !settings.autosave)">
      Autosave
    </MyToggle>
    <MyToggle
      :value="settings.use_saved_ticks"
      :click="() => (settings.use_saved_ticks = !settings.use_saved_ticks)"
    >
      Use saved time
    </MyToggle>
  </MyModal>
</template>

<script setup lang="ts">
import { ref, watch, computed } from "vue"
import { useStore } from "@store"
import MyToggle from "@c/util/MyToggle.vue"
import MyModal from "./MyModal.vue"
import { UserSettings } from "@p/index"

let store = useStore()
let wasm = computed(() => store.state.wasm)
let settings = ref<UserSettings>(store.state.user_settings)

watch(settings, save_settings, { deep: true })

function save_settings() {
  wasm.value.set_user_settings(settings.value)
}
</script>

<style scoped></style>
