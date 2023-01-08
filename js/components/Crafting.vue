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
      <Action category="Crafting" :item="crafting" />
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

/* let wasm = computed(() => store.state.wasm) */
</script>

<style scoped></style>
