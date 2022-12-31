<template>
  <div>
    <h4 class="section-header">Debug</h4>
    <div class="section">
      <div>
        <select>
          <option v-for="name in presets" :key="name" @click="wasm.set_preset_saves(name)">
            {{ name }}
          </option>
        </select>
      </div>
      <button @click="wasm.save">Save</button>
      <br />
      <button @click="wasm.load">Load</button>
      <br />
      <button @click="wasm.hard_reset">Hard Reset</button>
      <br />
      <hr />
      <button @click="wasm.set_gamespeed(1)">Set GameSpeed 1</button>
      <br />
      <button @click="wasm.set_gamespeed(10)">Set GameSpeed 10</button>
      <br />
      <button @click="wasm.set_gamespeed(30)">Set GameSpeed 30</button>
      <br />
      <button @click="wasm.set_gamespeed(100)">Set GameSpeed 100</button>
      <br />
      <button @click="wasm.set_gamespeed(1000)">Set GameSpeed 1000</button>
      <br />
      <hr />
      <button @click="tick">Tick</button>
      <br />
      <button @click="update">Update</button>
      <br />
      <MyToggle :value="world.status.auto_rebirth" :click="wasm.toggle_auto_rebirth">
        <br />
        Auto rebirth
      </MyToggle>
      <hr />
      <div>
        <div class="column-flex debug-printing">
          <span style="text-align: center"> Debug Printing </span>
          <button @click="print_frontend_debug_world">Frontend World</button>
          <button @click="wasm.print_debug_intermediate">Intermediate</button>
          <button @click="wasm.print_debug_state">State</button>
          <button @click="wasm.print_debug_meta">Meta</button>
          <button @click="wasm.print_debug_history">History</button>
          <button @click="wasm.print_debug_action_queue">Action Queue</button>
        </div>
      </div>
      <hr />
      <button @click="wasm.debug_die">Force death</button>
      <br />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useStore } from "@store"
import MyToggle from "@c/util/MyToggle.vue"

let store = useStore()
let wasm = computed(() => store.state.wasm)
let world = computed(() => store.state.world)
let presets = wasm.value.get_preset_saves()

function print_frontend_debug_world() {
  console.log(JSON.parse(JSON.stringify(world.value)))
}
function tick() {
  wasm.value.single_tick()
}
function update() {
  store.commit("update_dynamic_data")
}
</script>
<style scoped>
.debug-printing {
  gap: 0.1rem;
}
.section {
  margin-bottom: 1rem;
  padding: 0.5rem 0.5rem 0.5rem 0.5rem;
  background-color: #333c4a;
  border-radius: 1rem 1rem 1rem 1rem;
  border-top: 0.4rem solid #1a202b;
  border-left: 0.4rem solid #1a202b;
  border-right: 0.4rem solid #1a202b;
  border-bottom: 0.4rem solid #1a202b;
}
</style>
