<template>
  <div class="spreader">
    <div class="i-section items-main">
      <div class="around-flex">
        <my-icon :icon="icons['Backpack']" class="i-title"> Items </my-icon>
      </div>
      <hr />
      <div class="between-flex">
        <div class="i-name i-header">Item</div>
        <div class="i-actions i-header">Amount</div>
      </div>
      <div v-for="item in items" :key="item.name" class="i-box">
        <div>
          <div class="between-flex">
            <div class="i-name">
              <MyPopper :content="item.description">
                {{ item.display_name }}
                <my-icon
                  v-if="item.can_use && item.amount > 0"
                  :icon="icons['Schedule']"
                  @click.prevent="use_item(item.name)"
                />
              </MyPopper>
            </div>
            <div class="i-actions">{{ item.amount }} / 10</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useStore } from "@store"
import { ItemTypes } from "@p/index"

/* <button @click.prevent="enqueue_item(item.name)">enqueue</button> */
let store = useStore()
let icons = computed(() => store.state.world.icons)
let items = computed(() =>
  store.state.world.items.filter((item) => {
    return item.is_visible
  })
)

let wasm = computed(() => store.state.wasm)
function use_item(name: ItemTypes) {
  wasm.value.use_item(name)
  store.commit("update_dynamic_data")
}
</script>

<style scoped>
.items-main {
  min-height: 20rem;
  max-height: 30rem;
}
.spreader {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}
</style>
