<template>
  <div class="i-section">
    <div class="around-flex">
      <my-icon :icon="icons['Collection']" class="i-title"> Collection </my-icon>
    </div>
    <hr />
    <div class="between-flex">
      <div class="i-skill i-header">Skill</div>
      <div class="i-name i-header">Collect</div>
      <div class="i-actions i-header">Action</div>
    </div>
    <div v-for="collection in collections" :key="collection.name" class="i-box">
      <Action category="Collection" :item="collection" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useStore } from "@store"
import Action from "@c/Action.vue"

let store = useStore()
let icons = computed(() => store.state.world.icons)
let collections = computed(() =>
  store.getters.current_floor.collections.filter((collection) => {
    if (collection.max_completions && collection.round_completions >= collection.max_completions) {
      return false
    }
    return collection.is_visible
  })
)

/* let wasm = computed(() => store.state.wasm) */
</script>

<style scoped></style>
