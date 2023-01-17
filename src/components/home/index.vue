<template>
  <div class="page">
    <div id="nav">
      <el-menu
          default-active="1">
        <el-menu-item index="1" @click="onModules">
          <span>模块列表</span>
        </el-menu-item>
        <el-menu-item index="2">
          <span>Table模板</span>
        </el-menu-item>
        <el-menu-item index="3" @click="onSpring">
          <span>Spring模板</span>
        </el-menu-item>
        <el-menu-item index="4">
          <span>Page模板</span>
        </el-menu-item>
        <el-menu-item index="5" @click="onOptions">
          <span>生成配置</span>
        </el-menu-item>
        <div style="flex-grow: 1"></div>
        <el-menu-item index="9">
          <span>设置</span>
        </el-menu-item>
      </el-menu>
    </div>
    <div class="content">
      <div data-tauri-drag-region id="title">
        <div id="window-actions">
          <el-icon :size="16">
            <Minus/>
          </el-icon>
          <el-icon :size="16">
            <FullScreen/>
          </el-icon>
          <el-icon :size="20">
            <Close/>
          </el-icon>
        </div>
      </div>
      <modules v-if="content === 'modules'"/>
      <options v-if="content === 'options' "/>
      <Spring v-if="content === 'spring'"/>
    </div>
  </div>
</template>

<script setup>
import {Coin, Connection, House, Operation, Postcard, Setting, Close, Minus, FullScreen} from '@element-plus/icons-vue'
import Modules from "./modules.vue";
import Options from "./options/index.vue";
import Spring from "./template/Spring.vue";
import {ref} from "vue";

const content = ref('modules')

function onSettingsClick() {
}

function onNewClick() {
}

function onOptions() {
  content.value = 'options'
}

function onModules() {
  content.value = 'modules'
}

function onSpring() {
  content.value = 'spring'
}
</script>

<style lang="scss" scoped>
.page {
  flex-direction: row;

  #title {
    height: 40px;
    align-items: center;

    #window-actions {
      float: right;
      height: 32px;
      width: 126px;
      display: flex;
      align-content: space-between;

      .el-icon {
        width: 42px;
        height: 32px;
        color: #4b4b4b;
        vertical-align: middle;
      }

      .el-icon:hover {
        background-color: rgba(0, 0, 0, 0.11);
      }
    }
  }

  #nav {
    .el-menu {
      display: flex;
      flex-direction: column;
      height: 100%;
      border: none;
      padding-bottom: 8px;
      .el-menu-item{
        --el-menu-item-height: 42px;
        --el-menu-base-level-padding: 16px;
        --el-menu-hover-bg-color: var(--el-color-info-light-9);
        margin: 8px 8px 0 8px;
        border-radius: 6px;
      }
      .el-menu-item.is-active {
        background: var(--el-color-primary-light-9);
      }
    }
  }

}
</style>