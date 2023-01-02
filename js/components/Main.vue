<template>
  <div>
    <HealthBar class="main-bar" />
    <div class="below-bar">
      <Skills class="skills-column" />
      <div class="main-container">
        <CurrentFloor class="action-column" />
        <div class="spreader">
          <div class="sub-container">
            <Items class="main-column" />
            <Actions class="main-column" />
          </div>
          <Info class="info-column" />
        </div>
      </div>
    </div>
    <button class="settings-button" @click="showSettings">
      <font-awesome-icon class="settings-icon" :icon="icon_name(icons['Settings'])" />
    </button>
  </div>
</template>

<script setup lang="ts">
/* import { computed, ref } from "vue"; */
import Info from "@c/Info.vue"
import HealthBar from "@c/HealthBar.vue"
import Skills from "@c/Skills.vue"
import Items from "@c/Items.vue"
import Actions from "@c/Actions.vue"
import CurrentFloor from "@c/CurrentFloor.vue"
import { icon_name } from "@util"

import { computed } from "vue"
import { useStore } from "@store"
import { $vfm } from "vue-final-modal"

let store = useStore()
let icons = computed(() => store.state.world.icons)

function showSettings() {
  $vfm.show("settings")
}
</script>

<style scoped>
.settings-icon {
  width: 1.5rem;
  height: 1.5rem;
}
.settings-button {
  z-index: 2000;
  position: fixed;
  bottom: 0.5rem;
  right: 0.5rem;
}
.main-bar {
  flex-grow: 0;
  flex-shrink: 0;
}
.main-item {
  flex-grow: 1;
  flex-basis: 0;
}
.sub-container {
  display: flex;
  flex-direction: row;
  align-items: stretch;
}
.below-bar {
  display: flex;
  flex-grow: 1;
  flex-direction: row;
  justify-content: left;
  align-items: stretch;
}
.main-container {
  display: flex;
  flex-grow: 1;
  flex-direction: row;
  justify-content: center;
  align-items: stretch;
}
.action-column {
  min-width: 22rem;
  padding: 1rem 1rem 1rem 1rem;
}
.main-column {
  min-width: 18rem;
  padding: 1rem 1rem 1rem 1rem;
}
.skills-column {
  padding: 1rem 1rem 1rem 1rem;
}
.info-column {
  width: 38rem;
  padding: 1rem 1rem 1rem 1rem;
}
.spreader {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}
</style>
