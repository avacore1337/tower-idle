<!DOCTYPE html>
<template>
  <div>
    <div class="menu-list">
      <hr />
      <div class="around-flex">
        <div>
          Total:
          <my-icon :icon="icons['Clock']" />
          <TimeDisplay :value="total_time" />
        </div>
        <my-icon :icon="icons['Tombstone']"> {{ world.status.reincarnation }} </my-icon>
        <div>
          Current:
          <my-icon :icon="icons['Clock']" />
          <TimeDisplay :value="current_time" />
        </div>
      </div>
      <button class="main-button" highlight @click="showHistoryModal">
        <my-icon :icon="icons['History']"> History </my-icon>
      </button>
      <button class="main-button" highlight @click="showAutomationModal">
        <my-icon :icon="icons['Automation']"> Automation </my-icon>
      </button>
      <button class="main-button" highlight @click="showGeneralSettingsModal">
        <my-icon :icon="icons['Settings']"> General settings </my-icon>
      </button>
      <div class="row-flex">
        <button class="multi-button" highlight @click="wasm.save">
          <my-icon :icon="icons['Save']"> Save </my-icon>
        </button>
      </div>
      <textarea
        id="textarea"
        v-model="save_text"
        placeholder="Paste save here"
        rows="6"
        max-rows="6"
      />
      <div class="row-flex">
        <button class="multi-button" highlight @click="import_save">
          <my-icon :icon="icons['Import']"> Import save </my-icon>
        </button>
        <button class="multi-button" highlight @click="export_save">
          <my-icon :icon="icons['Export']"> Export save </my-icon>
        </button>
        <button class="multi-button" highlight @click="hard_reset">
          <my-icon :icon="icons['Trash']"> Wipe Save </my-icon>
        </button>
      </div>
      <div class="row-flex">
        <button class="multi-button" highlight @click="download_save">
          <my-icon :icon="icons['Download']"> Download save </my-icon>
        </button>
        <button class="multi-button" highlight @click="upload_save">
          <my-icon :icon="icons['Download']"> TODO upload save </my-icon>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue"
import { $vfm } from "vue-final-modal"
import { useStore } from "@store"
import TimeDisplay from "@c/util/TimeDisplay.vue"
import { downloadFile } from "../../util2.js"

let store = useStore()
let wasm = computed(() => store.state.wasm)
let world = computed(() => store.state.world)
let history = computed(() => store.state.history)
let icons = computed(() => store.state.world.icons)

let current_time = computed(() => store.state.world.status.tick / 30)
let total_time = computed(() => (history.value.total_ticks + store.state.world.status.tick) / 30)
let save_text = ref("")

function showHistoryModal() {
  $vfm.show("history")
}
function showAutomationModal() {
  $vfm.show("automation")
}
function showGeneralSettingsModal() {
  $vfm.show("general")
}

function download_save() {
  downloadFile(`gamesave_` + Date.now() + `.txt`, wasm.value.export_save())
}
/* function import_save_file(event) { */
/*   // TODO: This is only on the frontend atm, it doesn't actually save the changes */
/*   let files = event.target.files */
/*   let f = files[0] */
/*   let reader = new FileReader() */
/*   reader.onload = (function (theFile) { */
/*   return function (e) { */
/*     let data = e.target.result */
/*     console.log(data) */
/*     wasm.value.import_save(data) */
/*   } */
/*   })(f) */
/*   reader.readAsText(f) */
/* } */
function import_save() {
  wasm.value.import_save(save_text.value)
}
function export_save() {
  save_text.value = wasm.value.export_save()
}
function hard_reset() {
  if (confirm("Are you sure you want to do a hard reset? This will wipe all of your progress.")) {
    wasm.value.hard_reset()
  }
}
</script>

<style scoped>
.main-button {
  height: 2.5rem;
  font-size: 1.25rem;
  display: flex;
  flex-direction: row;
  justify-content: center;
}
.menu-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  flex: 1;
  /* justify-content: center; */
  /* align-items: center; */
}
.multi-button {
  display: flex;
  flex-direction: row;
  justify-content: center;
  height: 2.5rem;
  font-size: 1.25rem;
  flex-grow: 1;
  flex-basis: 0;
}
hr {
  /* flex-grow: 1; */
  width: 100%;
}
</style>
