<template>
  <div v-if="loaded" style="height: 100%">
    <Modals />
    <div class="app">
      <Main class="main" />
      <Debug v-if="enable_debug" class="debug" />
    </div>
  </div>
</template>

<script setup lang="ts">
import Main from "@c/Main.vue"
import Debug from "@c/Debug.vue"
import Modals from "@c/modals/Modals.vue"
import { computed, ref, onMounted } from "vue"
import { useStore } from "@store"
import { $vfm } from "vue-final-modal"

let enable_debug = true
let store = useStore()
let wasm = computed(() => store.state.wasm)
let loaded = ref(false)

onMounted(() => {
  loaded.value = true
  set_keyboard_listeners()

  setInterval(function () {
    store.commit("tick")
  }, 1000 / 30)
})

function set_keyboard_listeners() {
  window.addEventListener("keydown", (event) => {
    if (event.code == "Space") {
      event.preventDefault()
      wasm.value.toggle_paused()
    }
  })
  window.addEventListener(
    "contextmenu",
    (ev) => {
      if (!ev.altKey) {
        ev.preventDefault()
      }
      return false
    },
    false
  )
}
</script>

<style scoped>
.app {
  display: flex;
  align-items: stretch;
  height: 100vh;
}
.debug {
  width: 200px;
  flex-shrink: 0;
}
.main {
  flex-grow: 1;
  flex-shrink: 0;
  height: 100%;
  display: flex;
  flex-direction: column;
}
</style>
