<template>
  <div>
    <div class="between-flex">
      <div class="i-skill">
        <my-icon :icon="item.icon" />
        <my-icon v-if="item.collect && item.collect.Mana" :icon="icons['Mana']" />
        <my-icon v-if="item.damage && item.damage.Dps" :icon="icons['ManaDrain']" />
        <my-icon v-if="item.damage && item.damage.End" :icon="icons['ManaDamageAfter']" />
      </div>
      <div class="i-name">
        <ActionPopper :item="item" />
      </div>
      <div class="i-actions between-flex">
        <my-icon
          :icon="icons['Play']"
          @click.prevent.left="wasm.prepend_action(category, item.name, 100)"
          @click.prevent.right="wasm.prepend_action(category, item.name, 1)"
        />
        <my-icon
          v-if="item.is_automatable"
          :icon="get_priority_icon(item)"
          @click.prevent="wasm.toggle_priority(category, item.name)"
        />
        <my-icon
          :icon="icons['Schedule']"
          @click.prevent.left="wasm.append_action(category, item.name, 100)"
          @click.prevent.right="wasm.append_action(category, item.name, 1)"
        />
      </div>
    </div>
    <div class="completion-bar">
      <MyProgressBar :value="item.completion_percentage" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useStore } from "@store"
import MyProgressBar from "@c/util/MyProgressBar.vue"
import ActionPopper from "@c/ActionPopper.vue"
import type { Icon } from "@p/index"

let store = useStore()
let icons = computed(() => store.state.world.icons)
let wasm = computed(() => store.state.wasm)

function get_priority_icon(item: any): Icon {
  return icons.value["Priority" + item.priority.toString()]
}

/* let props = defineProps({ */
defineProps({
  item: {
    type: Object,
    required: true,
  },
  category: {
    type: String,
    required: true,
  },
})
</script>

<style scoped>
.completion-bar {
  margin-top: 0.1rem;
}
</style>
