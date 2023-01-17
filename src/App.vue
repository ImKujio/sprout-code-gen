<template>
  <div id="body">
    <Home @switchPage="switchPage" v-if="page === 'home'"/>
    <Module v-if="page === 'module' "/>
  </div>
</template>

<script setup>
import Home from "./components/home/index.vue";
import Module from "./components/module/index.vue";
import {ref} from "vue";

const page = ref('home')

function toHome() {
  page.value = 'home'
}

function toModule() {
  page.value = 'module'
}
</script>

<style lang="scss" scoped>

#body {
  display: flex;
  width: 100%;
  height: 100%;
  flex-direction: column;
  border: 1px #72767b;

  .el-menu {
    user-select: none;

    .el-menu-item {
      line-height: 32px;
      height: 32px;
    }
  }

}
</style>
