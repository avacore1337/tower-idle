<template>
  <div class="actions-container">
    <Collection />
    <Crafting />
    <Exploration />
  </div>
</template>

<script setup lang="ts">
/* import { computed } from "vue" */
/* import { useStore } from "@store" */
import Collection from "@c/Collection.vue"
import Crafting from "@c/Crafting.vue"
import Exploration from "@c/Exploration.vue"

/* let store = useStore() */
/* let current_floor = computed(() => store.getters.current_floor) */
/* let wasm = computed(() => store.state.wasm) */
</script>

<style scoped>
.actions-container {
  display: flex;
  flex-direction: column;
  max-width: 200px;
}
</style>
