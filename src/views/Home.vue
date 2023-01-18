<template>
  <div class="flex-rwo-fill">
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
    <div class="flex-col-fill" style="margin-left: 8px;">
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
import {Close, Minus, FullScreen} from '@element-plus/icons-vue'
</script>

<style lang="scss" scoped>

</style>