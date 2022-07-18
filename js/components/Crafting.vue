<template>
  <div class="i-section">
    <div class="around-flex">
      <my-icon :icon="icons['Crafting']" class="i-title"> Crafting </my-icon>
    </div>
    <hr />
    <div class="between-flex">
      <div class="i-skill i-header">Skill</div>
      <div class="i-name i-header">Goal</div>
      <div class="i-actions i-header">Action</div>
    </div>
    <div v-for="crafting in craftings" :key="crafting.name" class="i-box">
      <Action
        :item="crafting"
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
let craftings = computed(() =>
  store.getters.current_floor.craftings.filter((crafting) => {
    return crafting.is_visible
  })
)

let wasm = computed(() => store.state.wasm)
function do_action(name: string, amount: number) {
  wasm.value.prepend_crafting(name, amount)
}
function toggle_priority(name: string) {
  wasm.value.toggle_priority_crafting(name)
}
function schedule_action(name: string, amount: number) {
  wasm.value.append_crafting(name, amount)
}
</script>

<style scoped></style>
