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
      <Action category="Exploration" :item="exploration" />
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
  store.getters.current_floor.explorations.filter((a) => a.is_visible)
)

/* let wasm = computed(() => store.state.wasm) */
</script>

<style scoped></style>
