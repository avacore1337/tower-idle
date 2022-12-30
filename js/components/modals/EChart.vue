<template>
  <v-chart class="chart" :option="option" />
</template>

<script setup lang="ts">
import { use } from "echarts/core"
import { CanvasRenderer } from "echarts/renderers"
import { PieChart } from "echarts/charts"
import { TitleComponent, TooltipComponent, LegendComponent } from "echarts/components"
import VChart, { THEME_KEY } from "vue-echarts"
import { provide, computed } from "vue"
use([CanvasRenderer, PieChart, TitleComponent, TooltipComponent, LegendComponent])

const props = defineProps({
  datas: { type: Object, required: true },
})

provide(THEME_KEY, "dark")
let option = computed(() => {
  return {
    tooltip: {
      trigger: "item",
      formatter: "{a} <br/>{b} : {c} ({d}%)",
    },
    series: [
      {
        name: "Levels gained",
        type: "pie",
        radius: "55%",
        center: ["50%", "60%"],
        data: props.datas,
        emphasis: {
          itemStyle: {
            shadowBlur: 10,
            shadowOffsetX: 0,
            shadowColor: "rgba(0, 0, 0, 0.5)",
          },
        },
      },
    ],
  }
})
</script>

<style scoped>
.chart {
  height: 200px;
  width: 200px;
}
</style>
