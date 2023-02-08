<template>
  <RemoveScript src="https://cdn.jsdelivr.net/npm/vditor@3.9.0/dist/index.min.js" />
  <h1>{{ item.path }}</h1>
  <div id="vditor"/>
</template>

<!--See => -->
<!--https://techvblogs.com/blog/how-to-load-an-external-script-in-vue-component、-->
<!--https://juejin.cn/post/7049217566144725029-->
<!--<script type="application/javascript" src="https://cdn.jsdelivr.net/npm/vditor@3.9.0/dist/index.min.js"></script>-->
<link href="https://cdn.jsdelivr.net/npm/vditor@3.9.0/dist/js/highlight.js/styles/dracula.min.css" rel="stylesheet">
<script setup lang="ts">
import {onMounted, onUpdated, reactive, ref} from 'vue';
import {get} from '../utils/request';
import Vditor from 'vditor';
// import 'vditor/dist/index.css';
import RemoveScript from '../components/RemoteScript.vue';


const vditor = ref<Vditor | null>(null);
// const item = reactive(window.opener.item)  // 不利于基于url直接访问资源
const param = location.href.split('?').pop().split('=')
const item = reactive({id: param[0], name: param[1]})

onMounted(() => {
  vditor.value = new Vditor('vditor', {
    mode: 'wysiwyg',
    after: () => {
      (async () => {
        get(`/show/${item.id}/${item.name}`).then(resp => {
          vditor.value!.setValue(resp.data);
          vditor.value.disabled()
        })
      })()
      document.getElementById('vditor').removeAttribute("style");
      document.getElementsByClassName("vditor-toolbar")[0].removeAttribute("style");
      document.getElementsByClassName("vditor-reset")[0].removeAttribute("style");
      document.getElementsByClassName("vditor-reset")[2].removeAttribute("style");
      document.getElementsByClassName("vditor-reset")[3].removeAttribute("style");

      // 临时
      document.getElementsByClassName("vditor-reset")[0].style = "opacity:1;";
    },
  });
});
onUpdated(() => {

})
</script>

<style lang="scss">
h1 {
  width: 60%;
  margin: 20px auto;
  min-width: 1240px;
}

#vditor {
  box-shadow: 0 0 20px rgb(208 208 208);
  min-width: 1240px;
  width: 60%;
  margin: auto;
  height: 100vh;
  border-radius: 6px 6px 6px 6px;

  .vditor-toolbar {
    height: 0;
    line-height: 2;
    border-radius: 6px 6px 0 0;
    padding-left: 170px;
  }

  .vditor-content {
    .vditor-wysiwyg {
      .vditor-reset {
        border-radius: 0 0 6px 6px;
        padding: 30px 170px;
      }
    }
  }
}

.vditor-content pre.vditor-reset:focus {
  background-color: white;
}

.vditor-ir .vditor-reset {
  border-radius: 0 0 6px 6px;
  padding: 10px 170px;
}

.vditor-sv .vditor-reset {
  border-radius: 0 0 6px 6px;
  padding: 10px 170px;
}

.vditor-toolbar .vditor-toolbar__item button svg {
  height: 22px;
  width: 22px;
}

.vditor-toolbar__item .vditor-tooltipped {
  width: auto;
}
</style>
