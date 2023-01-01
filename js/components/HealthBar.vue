<!DOCTYPE html>
<template>
  <div class="health-progress" style="position: relative">
    <div class="health-bar" :style="{ width: current_health_percentage + '%' }"></div>
    <div class="progress-text">
      <my-icon :icon="icons['Clock']">
        <span style="margin-right: 3.2rem">
          <TimeDisplay :value="current_time" />
        </span>
      </my-icon>
      <my-icon :icon="icons['Health']">
        <span style="margin-right: 3.2rem">
          <FormatNumber :value="current_health" /> / <FormatNumber :value="max_health" />
        </span>
      </my-icon>
      <my-icon :icon="icons['HealthDrain']">
        <FormatNumber :value="health_drain" />
      </my-icon>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useStore } from "@store"
import { icon_name } from "@util"
import FormatNumber from "@c/util/FormatNumber.vue"
import TimeDisplay from "@c/util/TimeDisplay.vue"

let store = useStore()
let icons = computed(() => store.state.world.icons)
let current_time = computed(() => store.state.world.status.tick / 30)
let current_health = computed(() => store.state.world.status.current_health)
let current_health_percentage = computed(() => store.state.world.status.current_health_percentage)
let max_health = computed(() => store.state.world.status.max_health)
let health_drain = computed(() => store.state.world.status.health_drain)
</script>

<style scoped>
.health-progress {
  width: 100%;
  background-color: #ddd;
  height: 2rem;
}

.health-bar {
  position: absolute;
  height: 2rem;
  background-color: #182fa3;
}

.progress-text {
  position: absolute;
  display: flex;

  flex-direction: row;
  justify-content: center;

  padding: 0.5rem;
  color: white;
  box-sizing: inherit;
  white-space: nowrap;
  width: 100%;
  left: 0;
  right: 0;
}
</style>
