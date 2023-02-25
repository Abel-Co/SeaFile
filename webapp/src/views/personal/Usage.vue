<template>
  <n-space>
    <n-space :item-style="{width:'600px',height:'500px'}">
      <v-chart class="chart" :option="option" autoresize/>
    </n-space>
    <n-space @click="change1">
      Hello
    </n-space>
    <n-space @click="change2">
      World
    </n-space>
  </n-space>
  <!--  <pre>{{ JSON.stringify(model, null, 2) }}</pre>-->
</template>

<script setup>
import { reactive, ref } from "vue"
import { use } from 'echarts/core'
import { SunburstChart } from 'echarts/charts'
import { CanvasRenderer } from 'echarts/renderers'
import { TitleComponent, TooltipComponent, LegendComponent, ToolboxComponent } from 'echarts/components'
import VChart, { THEME_KEY } from 'vue-echarts'
import { get } from '../../utils/request'

use([CanvasRenderer, SunburstChart, TitleComponent, TooltipComponent, LegendComponent, ToolboxComponent])

const data = reactive([])
const data1 = [{label:'Flora',itemStyle:{color:'#da0d68'},children:[{label:'Black Tea',value:1,itemStyle:{color:'#975e6d'}},{label:'Floral',itemStyle:{color:'#e0719c'},children:[{label:'Chamomile',value:1,itemStyle:{color:'#f99e1c'}},{label:'Rose',value:1,itemStyle:{color:'#ef5a78'}},{label:'Jasmine',value:1,itemStyle:{color:'#f7f1bd'}}]}]},{label:'Fruity',itemStyle:{color:'#da1d23'},children:[{label:'Berry',itemStyle:{color:'#dd4c51'},children:[{label:'Blackberry',value:1,itemStyle:{color:'#3e0317'}},{label:'Raspberry',value:1,itemStyle:{color:'#e62969'}},{label:'Blueberry',value:1,itemStyle:{color:'#6569b0'}},{label:'Strawberry',value:1,itemStyle:{color:'#ef2d36'}}]},{label:'Dried Fruit',itemStyle:{color:'#c94a44'},children:[{label:'Raisin',value:1,itemStyle:{color:'#b53b54'}},{label:'Prune',value:1,itemStyle:{color:'#a5446f'}}]},{label:'Other Fruit',itemStyle:{color:'#dd4c51'},children:[{label:'Coconut',value:1,itemStyle:{color:'#f2684b'}},{label:'Cherry',value:1,itemStyle:{color:'#e73451'}},{label:'Pomegranate',value:1,itemStyle:{color:'#e65656'}},{label:'Pineapple',value:1,itemStyle:{color:'#f89a1c'}},{label:'Grape',value:1,itemStyle:{color:'#aeb92c'}},{label:'Apple',value:1,itemStyle:{color:'#4eb849'}},{label:'Peach',value:1,itemStyle:{color:'#f68a5c'}},{label:'Pear',value:1,itemStyle:{color:'#baa635'}}]},{label:'Citrus Fruit',itemStyle:{color:'#f7a128'},children:[{label:'Grapefruit',value:1,itemStyle:{color:'#f26355'}},{label:'Orange',value:1,itemStyle:{color:'#e2631e'}},{label:'Lemon',value:1,itemStyle:{color:'#fde404'}},{label:'Lime',value:1,itemStyle:{color:'#7eb138'}}]}]},{label:'Sour/\nFermented',itemStyle:{color:'#ebb40f'},children:[{label:'Sour',itemStyle:{color:'#e1c315'},children:[{label:'Sour Aromatics',value:1,itemStyle:{color:'#9ea718'}},{label:'Acetic Acid',value:1,itemStyle:{color:'#94a76f'}},{label:'Butyric Acid',value:1,itemStyle:{color:'#d0b24f'}},{label:'Isovaleric Acid',value:1,itemStyle:{color:'#8eb646'}},{label:'Citric Acid',value:1,itemStyle:{color:'#faef07'}},{label:'Malic Acid',value:1,itemStyle:{color:'#c1ba07'}}]},{label:'Alcohol/\nFremented',itemStyle:{color:'#b09733'},children:[{label:'Winey',value:1,itemStyle:{color:'#8f1c53'}},{label:'Whiskey',value:1,itemStyle:{color:'#b34039'}},{label:'Fremented',value:1,itemStyle:{color:'#ba9232'}},{label:'Overripe',value:1,itemStyle:{color:'#8b6439'}}]}]},{label:'Green/\nVegetative',itemStyle:{color:'#187a2f'},children:[{label:'Olive Oil',value:1,itemStyle:{color:'#a2b029'}},{label:'Raw',value:1,itemStyle:{color:'#718933'}},{label:'Green/\nVegetative',itemStyle:{color:'#3aa255'},children:[{label:'Under-ripe',value:1,itemStyle:{color:'#a2bb2b'}},{label:'Peapod',value:1,itemStyle:{color:'#62aa3c'}},{label:'Fresh',value:1,itemStyle:{color:'#03a653'}},{label:'Dark Green',value:1,itemStyle:{color:'#038549'}},{label:'Vegetative',value:1,itemStyle:{color:'#28b44b'}},{label:'Hay-like',value:1,itemStyle:{color:'#a3a830'}},{label:'Herb-like',value:1,itemStyle:{color:'#7ac141'}}]},{label:'Beany',value:1,itemStyle:{color:'#5e9a80'}}]},{label:'Other',itemStyle:{color:'#0aa3b5'},children:[{label:'Papery/Musty',itemStyle:{color:'#9db2b7'},children:[{label:'Stale',value:1,itemStyle:{color:'#8b8c90'}},{label:'Cardboard',value:1,itemStyle:{color:'#beb276'}},{label:'Papery',value:1,itemStyle:{color:'#fefef4'}},{label:'Woody',value:1,itemStyle:{color:'#744e03'}},{label:'Moldy/Damp',value:1,itemStyle:{color:'#a3a36f'}},{label:'Musty/Dusty',value:1,itemStyle:{color:'#c9b583'}},{label:'Musty/Earthy',value:1,itemStyle:{color:'#978847'}},{label:'Animalic',value:1,itemStyle:{color:'#9d977f'}},{label:'Meaty Brothy',value:1,itemStyle:{color:'#cc7b6a'}},{label:'Phenolic',value:1,itemStyle:{color:'#db646a'}}]},{label:'Chemical',itemStyle:{color:'#76c0cb'},children:[{label:'Bitter',value:1,itemStyle:{color:'#80a89d'}},{label:'Salty',value:1,itemStyle:{color:'#def2fd'}},{label:'Medicinal',value:1,itemStyle:{color:'#7a9bae'}},{label:'Petroleum',value:1,itemStyle:{color:'#039fb8'}},{label:'Skunky',value:1,itemStyle:{color:'#5e777b'}},{label:'Rubber',value:1,itemStyle:{color:'#120c0c'}}]}]},{label:'Roasted',itemStyle:{color:'#c94930'},children:[{label:'Pipe Tobacco',value:1,itemStyle:{color:'#caa465'}},{label:'Tobacco',value:1,itemStyle:{color:'#dfbd7e'}},{label:'Burnt',itemStyle:{color:'#be8663'},children:[{label:'Acrid',value:1,itemStyle:{color:'#b9a449'}},{label:'Ashy',value:1,itemStyle:{color:'#899893'}},{label:'Smoky',value:1,itemStyle:{color:'#a1743b'}},{label:'Brown, Roast',value:1,itemStyle:{color:'#894810'}}]},{label:'Cereal',itemStyle:{color:'#ddaf61'},children:[{label:'Grain',value:1,itemStyle:{color:'#b7906f'}},{label:'Malt',value:1,itemStyle:{color:'#eb9d5f'}}]}]},{label:'Spices',itemStyle:{color:'#ad213e'},children:[{label:'Pungent',value:1,itemStyle:{color:'#794752'}},{label:'Pepper',value:1,itemStyle:{color:'#cc3d41'}},{label:'Brown Spice',itemStyle:{color:'#b14d57'},children:[{label:'Anise',value:1,itemStyle:{color:'#c78936'}},{label:'Nutmeg',value:1,itemStyle:{color:'#8c292c'}},{label:'Cinnamon',value:1,itemStyle:{color:'#e5762e'}},{label:'Clove',value:1,itemStyle:{color:'#a16c5a'}}]}]},{label:'Nutty/\nCocoa',itemStyle:{color:'#a87b64'},children:[{label:'Nutty',itemStyle:{color:'#c78869'},children:[{label:'Peanuts',value:1,itemStyle:{color:'#d4ad12'}},{label:'Hazelnut',value:1,itemStyle:{color:'#9d5433'}},{label:'Almond',value:1,itemStyle:{color:'#c89f83'}}]},{label:'Cocoa',itemStyle:{color:'#bb764c'},children:[{label:'Chocolate',value:1,itemStyle:{color:'#692a19'}},{label:'Dark Chocolate',value:1,itemStyle:{color:'#470604'}}]}]},{label:'Sweet',itemStyle:{color:'#e65832'},children:[{label:'Brown Sugar',itemStyle:{color:'#d45a59'},children:[{label:'Molasses',value:1,itemStyle:{color:'#310d0f'}},{label:'Maple Syrup',value:1,itemStyle:{color:'#ae341f'}},{label:'Caramelized',value:1,itemStyle:{color:'#d78823'}},{label:'Honey',value:1,itemStyle:{color:'#da5c1f'}}]},{label:'Vanilla',value:1,itemStyle:{color:'#f89a80'}},{label:'Vanillin',value:1,itemStyle:{color:'#f37674'}},{label:'Overall Sweet',value:1,itemStyle:{color:'#e75b68'}},{label:'Sweet Aromatics',value:1,itemStyle:{color:'#d0545f'}}]}];
const data2 = [{ value: 60, id: 'Direct', itemStyle: { color: 'none' } }, {value: 260, id: 'Direct'}, {value: 160, id: 'Direct'}, { value: 360, id: 'Direct', children: [{ value: 350, id: 'Email', children: [{ value: 340, id: 'Ad Networks', children: [{ value: 330, id: 'Video Ads', children: [{ value: 320, id: 'Search Engines', children: [{ value: 270, id: 'Search Engines', children: [{ value: 260, id: 'Direct', children: [{ value: 250, id: 'Email', children: [{ value: 240, id: 'Ad Networks', children: [{ value: 230, id: 'Video Ads', children: [{ value: 220, id: 'Search Engines', children: [{ value: 210, id: 'Search Engines', children: [{ value: 200, id: 'Search Engines' }] }] }] }] }] }] }] }] }] }] }] }] }]
// docs: const data2 = [{value:total-(sum(第一层value)),itemStyle:{color:'none'}}, { id: 123, path: '/home/abel/xxx', value: 60, label: 'abc.txt', kind:'Folder/File', itemStyle: { color: 'File? #7F7F7F' }, children: [] }]
data.push(...data2)
const change1 = () => data.splice(0, 10000, ...data1)
const change2 = () => data.splice(0, 10000, ...data2)

const levels = [];
// for (let i = 0; i < 5; i++) {
//   const o = 14, p = 12, x = o + i * p, y = x + p
//   levels.push({ r0: `${x}%`, r: `${y}%` })
// }
// console.log(JSON.stringify(levels))
// const o2 = parseInt(levels[levels.length-1].r.replace("%", ''))
// for (let i = 0; i < 8; i++) {
//   const o = o2, p = 2, x = o + i * p, y = x + p
//   levels.push({ r0: `${x}%`, r: `${y}%` })
// }
// console.log(JSON.stringify(levels))

// levels.push({r0:'14%', r: '26%'})
const level = {r0: '14%', r: '26%'};for (let i = 0; i< 13; i++) {levels.push(level)}

const option = reactive({
  backgroundColor: "rgba(255,255,255,0)",
  toolbox: {
    feature: {
      // saveAsImage: {},
    },
  },
  series: {
    type: "sunburst",
    data: data,
    sort: "value",
    emphasis: { focus: "none", },
    label: { show: false },
    levels: [{r0: 0, r:'14%'}, ...levels],
  },
})
</script>





















