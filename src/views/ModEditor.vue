<template>
  <content-title>
    <span id="title">模块</span>
    <el-select v-model="curModule" :suffix-icon="null" placeholder="选择模块">
      <el-option v-for="item in modules" :value="item.name" :label="item.name"/>
    </el-select>
  </content-title>
</template>

<script setup>

import ContentTitle from "../components/ContentTitle.vue";
import {Data} from "../api/data.js";
import {onActivated, ref} from "vue";

const modules = ref(null)
const curModule = ref(null)

function getModules(){
  Data.Modules.get().then(res => {
    modules.value = res
  })
}
getModules()

onActivated(() => {
  getModules()
})

</script>

<style scoped>
#title {
  font-weight: bold;
  font-size: var(--el-font-size-base);
  margin-top: 8px;
  margin-left: 15px;
  margin-right: 8px;
  color: var(--el-color-primary);
}
</style>