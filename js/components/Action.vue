<template>
  <div>
    <div class="between-flex">
      <div class="i-skill">
        <my-icon :icon="item.icon" />
      </div>
      <div class="i-name">
        <ActionPopper :item="item" />
      </div>
      <div class="i-actions between-flex">
        <my-icon
          :icon="icons['Play']"
          @click.prevent.left="do_action(item.name, 100)"
          @click.prevent.right="do_action(item.name, 1)"
        />
        <my-icon
          v-if="item.is_automatable"
          :icon="get_priority_icon(item)"
          @click.prevent="toggle_priority(item.name)"
        />
        <my-icon
          :icon="icons['Schedule']"
          @click.prevent.left="schedule_action(item.name, 100)"
          @click.prevent.right="schedule_action(item.name, 1)"
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
/* let wasm = computed(() => store.state.wasm) */

function get_priority_icon(item: any): Icon {
  return icons.value["Priority" + item.priority.toString()]
}

/* let props = defineProps({ */
defineProps({
  item: {
    type: Object,
    required: true,
  },
  do_action: {
    type: Function,
    required: true,
  },
  toggle_priority: {
    type: Function,
    required: true,
  },
  schedule_action: {
    type: Function,
    required: true,
  },
})
</script>

<style scoped>
.completion-bar {
  margin-top: 0.1rem;
}
</style>
