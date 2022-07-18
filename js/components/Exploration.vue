<template>
  <div class="i-section">
    <div class="around-flex">
      <my-icon :icon="icons['Exploration']" class="i-title"> Exploration </my-icon>
    </div>
    <hr />
    <div class="between-flex">
      <div class="i-skill i-header">Skill</div>
      <div class="i-name i-header">Goal</div>
      <div class="i-actions i-header">Action</div>
    </div>
    <div v-for="exploration in explorations" :key="exploration.name" class="i-box">
      <Action
        :item="exploration"
        :do_action="do_action"
        :toggle_priority="toggle_priority"
        :schedule_action="schedule_action"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useStore } from "@store"
import Action from "@c/Action.vue"

let store = useStore()
let icons = computed(() => store.state.world.icons)
let explorations = computed(() =>
  store.getters.current_floor.explorations.filter((exploration) => {
    return exploration.is_visible
  })
)

let wasm = computed(() => store.state.wasm)
function do_action(name: string, amount: number) {
  wasm.value.prepend_exploration(name, amount)
}
function toggle_priority(name: string) {
  wasm.value.toggle_priority_exploration(name)
}
function schedule_action(name: string, amount: number) {
  wasm.value.append_exploration(name, amount)
}
</script>

<style scoped></style>
