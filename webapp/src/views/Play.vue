<template>
  <div>
    <VideoPlay
        v-bind="options"
        poster="https://cdn.jsdelivr.net/gh/xdlumia/files/video-play/ironMan.jpg"
    />
  </div>
</template>

<script setup>
import { onMounted, reactive } from 'vue'
import { videoPlay } from 'vue3-video-play'
import "vue3-video-play/dist/style.css"

const VideoPlay = videoPlay
// const item = reactive(window.opener.item)  // 不利于基于url直接访问资源
const param = location.href.split('?').pop().split('=')
const item = reactive({id: param[0], name: param[1]})
const options = reactive({
  width: "80%",     // 播放器高度
  height: "80%",    // 播放器高度
  color: "#409eff", // 主题色
  title: item.name, // 视频名称
  src: `${location.origin}/visit/${item.id}/${item.name}`, // 视频源
  muted: false,     // 静音
  webFullScreen: true,
  speedRate: ["0.75", "1.0", "1.25", "1.5", "2.0"],   // 播放倍速
  autoPlay: true,   // 自动播放
  loop: false,      // 循环播放
  mirror: false,    // 镜像画面
  ligthOff: false,  // 关灯模式
  volume: 0.3,      // 默认音量大小
  control: true,    // 是否显示控制
  controlBtns: [
    "audioTrack",
    "quality",
    "speedRate",
    "volume",
    "setting",
    "pip",
    "pageFullScreen",
    "fullScreen",
  ],                // 显示所有按钮,
})
const openVedio = (url, title) => {
  options.title = title
  options.src = getRootPath() + url
}

defineExpose({
  openVedio,
})
onMounted(() => {
  let interval = setInterval(() => {
    let dom = document.getElementsByClassName("d-icon iconfont icon-volume-mute")
    if (dom?.length > 0) {
      clearInterval(interval)
      dom[0].click()
    }
    console.log('hello wold', interval)
  }, 150)
})
</script>