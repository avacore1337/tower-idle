<template>
  <div style="min-height: 20rem">
    <div class="i-section">
      <div class="i-title">Actions</div>
      <hr />

      <div class="action-box titles">
        <span class="order-width"> <b>Order</b> </span>
        <span class="action-width"> <b>Action</b> </span>
        <my-icon
          class="button-width"
          :icon="icons['Trash']"
          @click.prevent="wasm.clear_all_actions()"
        />
      </div>
      <div v-if="!no_actions" class="actions_list">
        <div v-for="(action, index) in actions" :key="action.action_key" class="action-box">
          <div class="row-flex order-width">
            <span class="number-width"> {{ index + 1 }} </span>
            <my-icon :icon="icons[action.skill_icon]" />
            <my-icon :icon="icons[action.category_icon]" />
          </div>
          <div class="text-box action-width">{{ action.display_name }}</div>
          <my-icon
            class="button-width"
            :icon="icons['Stop']"
            @click.prevent="wasm.lower_action_count(index)"
          />
        </div>
      </div>
      <div v-if="no_actions">Action queue is empty</div>
      <div v-if="!is_paused">
        <button @click="wasm.toggle_paused">Pause</button>
      </div>
      <div v-if="is_paused">
        <button @click="wasm.toggle_paused">Resume</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useStore } from "@store"

let store = useStore()
let wasm = computed(() => store.state.wasm)
let is_paused = computed(() => store.state.user_settings.paused)
let icons = computed(() => store.state.world.icons)
let actions = computed(() => store.state.action_queue)
let no_actions = computed(() => store.state.action_queue.length === 0)
</script>

<style scoped>
.number-width {
  width: 2rem;
}
.order-width {
  width: 4.5rem;
}
.action-width {
  width: 11rem;
}
.button-width {
  width: 1rem;
}
.titles {
  margin-bottom: 0.3rem;
}
.action-box {
  /* border: 1px solid white; */
  display: flex;
  min-width: 16rem;
  flex-direction: row;
  gap: 0.4rem;
  justify-content: left;
}
.text-box {
  display: flex;
  justify-content: left;
}
.actions_list {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
  overflow: scroll;
  max-height: 30rem;
}
</style>
