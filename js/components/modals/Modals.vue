<!DOCTYPE html>
<template>
  <div>
    <DeathModal />
    <MyModal
      name="settings"
      title="Settings"
      @closed="addEscapeListener"
      @opened="removeEscapeListener"
    >
      <Settings />
    </MyModal>

    <History />
    <Automation />
    <GeneralSettings />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, computed } from "vue"
import { $vfm } from "vue-final-modal"
import { useStore } from "@store"
import Settings from "@c/modals/Settings.vue"
import History from "@c/modals/History.vue"
import Automation from "@c/modals/Automation.vue"
import DeathModal from "@c/modals/DeathModal.vue"
import GeneralSettings from "@c/modals/GeneralSettings.vue"
import MyModal from "./MyModal.vue"
let store = useStore()
let is_open = ref(false)
watch(is_open, () => store.commit("update_history"))

onMounted(() => {
  addEscapeListener()
})

function addEscapeListener() {
  window.addEventListener("keydown", handleEscape)
}

function removeEscapeListener() {
  window.removeEventListener("keydown", handleEscape)
}

function handleEscape(event) {
  if (event.code == "Escape") {
    $vfm.show("settings")
  }
}
</script>

<style scoped>
:deep(.modal-container) {
  display: flex;
  justify-content: center;
  align-items: center;
}
:deep(.modal-content) {
  position: relative;
  display: flex;
  flex-direction: column;
  max-height: 90%;
  overflow-y: scroll;
  margin: 0 1rem;
  padding: 1rem;
  border: 1px solid #e2e8f0;
  border-radius: 0.25rem;
  background: #444;
  min-width: 40rem;
  /* min-height: 30rem; */
}
</style>
