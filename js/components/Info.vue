<template>
  <div>
    <h3>Events</h3>
    <div class="log-area" ref="text_area">
      <span v-for="(text, index) in info_texts" :key="index">
        {{ text }}
        <hr />
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue"
import { useStore } from "@store"
let store = useStore()
let icons = computed(() => store.state.world.icons)
let info_texts = computed(() => store.state.messages)

let text_area: any = ref(null)
watch(info_texts, scroll_down, { deep: true, flush: "post" })

function scroll_down(a, b) {
  text_area.value.scrollTop = text_area.value.scrollHeight
}
</script>

<style scoped>
.log-area {
  padding: 0.2rem;
  width: 100%;
  border: 1px solid lightgray;
  height: 12rem;
  overflow-y: scroll;
  white-space: pre-line;
}
</style>
