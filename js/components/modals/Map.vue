<template>
  <v-chart class="chart" :option="option" style="height: 600px; width: 600px" />
</template>

<script setup lang="ts">
import type { Edge, Node, MapData } from "@p/index"
import { use } from "echarts/core"
import { CanvasRenderer } from "echarts/renderers"
import { GraphChart } from "echarts/charts"
import { TitleComponent, TooltipComponent, LegendComponent } from "echarts/components"
import VChart, { THEME_KEY } from "vue-echarts"
import { Ref, ref, provide, watch, computed } from "vue"
import Rand, { PRNG } from "rand-seed"
import { useStore } from "@store"
use([CanvasRenderer, GraphChart, TitleComponent, TooltipComponent, LegendComponent])

let store = useStore()
let wasm = computed(() => store.state.wasm)

const props = defineProps({
  datas: { type: Object, required: true },
})

/* function createNodes(count: number) { */
/*   const rand = new Rand("1236") */
/*   var nodes: Node[] = [] */
/*   for (var i = 0; i < count; i++) { */
/*     nodes.push({ */
/*       name: i + "", */
/*       x: i * 10, */
/*       y: i * 10 + 10 * rand.next(), */
/*     }) */
/*   } */
/*   return nodes */
/* } */

/* function createEdges(count: number) { */
/*   var edges: Edge[] = [] */
/*   edges.push({ source: "0", target: "1" }) */
/*   edges.push({ source: "1", target: "2" }) */
/*   edges.push({ source: "1", target: "5" }) */
/*   edges.push({ source: "2", target: "3" }) */
/*   edges.push({ source: "3", target: "4" }) */
/*   edges.push({ source: "4", target: "7" }) */
/*   edges.push({ source: "5", target: "6" }) */
/*   edges.push({ source: "6", target: "7" }) */
/*   edges.push({ source: "7", target: "8" }) */
/*   edges.push({ source: "8", target: "9" }) */
/*   return edges */
/* } */

/* var datas: Ref<MapData[]> = ref([ */
/*   { */
/*     nodes: createNodes(8 + 2), */
/*     edges: createEdges(8 + 2), */
/*   }, */
/* ]) */
/* var datas: Ref<MapData[]> = ref([wasm.value.get_map_for_floor(store.getters.current_floor.name)]) */

provide(THEME_KEY, "dark")
let option = computed(() => {
  let name = store.getters.current_floor.name
  console.log("DATAS raw: ", props.datas)
  console.log("DATAS: ", JSON.parse(JSON.stringify(props.datas)))
  return {
    tooltip: {
      trigger: "item",
      formatter: "Floor: " + name + " <br/>{b} : {c} ",
    },
    series: props.datas.map(function (item, idx) {
      return {
        type: "graph",
        /* layout: "force", */
        animation: false,
        data: item.nodes,

        label: {
          show: true,
        },
        // left: (idx % 4) * 25 + '%',
        // top: Math.floor(idx / 4) * 25 + '%',
        width: "25%",
        height: "25%",

        symbol: "roundRect",

        symbolSize: 50,
        force: {
          // initLayout: 'circular',
          gravity: 1,
          repulsion: 50,
          edgeLength: 50,
        },
        edges: item.edges,
      }
    }),
  }
})
</script>

<style scoped>
.chart {
  height: 600px;
  width: 600px;
}
</style>
