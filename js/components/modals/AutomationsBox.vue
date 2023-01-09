<!DOCTYPE html>
<template>
  <div class="row-flex">
    <div v-for="category in categories" :key="category.name" class="action-box">
      <div class="i-title">
        {{ category.name }}
      </div>
      <hr />
      <div class="action-lister">
        <div class="action-row titles">
          <span class="icon-width"> Skill </span>
          <span class="action-width"> Name </span>
          <span class="toggle-box-width"> Auto </span>
        </div>
        <div v-for="action in category.actions" :key="action.name" class="action-row">
          <my-icon class="icon-width" :icon="action.icon" />

          <span class="action-width between-flex">
            {{ action.display_name }}
            <span v-if="!action.is_automatable">
              {{ action.completion_count }} / {{ action.automate_limit }}
            </span>
          </span>
          <div class="row-flex toggle-box-width">
            <my-icon
              v-if="action.is_automatable"
              :icon="get_priority_icon(action)"
              @click.prevent="wasm.toggle_priority(category.name, action.name)"
            />
            <my-icon
              v-if="action.favourite"
              :icon="icons['Favourite']"
              @click.prevent="wasm.toggle_favourite(category.name, action.name)"
            />
            <my-icon
              v-if="!action.favourite"
              :icon="icons['NotFavourite']"
              @click.prevent="wasm.toggle_favourite(category.name, action.name)"
            />
            <my-icon v-if="!action.is_automatable" :icon="icons['Lock']" />
            <my-icon v-if="action.round_completions > 0" :icon="icons['Check']" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Icon } from "@p/index"
import { PropType, computed } from "vue"
import { useStore } from "@store"
import { Category } from "@state"

const props = defineProps({
  categories: { type: Object as PropType<Category[]>, required: true },
})

let store = useStore()
let wasm = computed(() => store.state.wasm)
let icons = computed(() => store.state.world.icons)

function get_priority_icon(item: any): Icon {
  return icons.value["Priority" + item.priority.toString()]
}
</script>

<style scoped>
.action-row {
  display: flex;
  flex-direction: row;
  gap: 0.5rem;
}
.titles {
  margin-bottom: 0.3rem;
}
.action-box {
  padding: 1rem;
  border: 1px solid white;
  width: 22rem;
}
.action-lister {
  display: flex;
  flex-direction: column;
  min-height: 16rem;
  gap: 0.3rem;
}
.icon-width {
  width: 2.5rem;
}
.action-width {
  width: 16.5rem;
}
.toggle-box-width {
  width: 3rem;
}
</style>
