<template>
  <div class="flex-row-fill">
    <div class="flex-col-fill" style="width: auto;flex: none">
      <img data-tauri-drag-region src="/logo.svg" class="logo" alt="logo"/>
      <el-menu
          style="flex: 1"
          default-active="0"
          @select="i => index = parseInt(i)">
        <el-menu-item index="0">
          <span>类信息</span>
        </el-menu-item>
        <el-menu-item index="1">
          <span>模板列表</span>
        </el-menu-item>
        <el-menu-item index="2">
          <span>类信息列</span>
        </el-menu-item>
        <el-menu-item index="3">
          <span>类属性列</span>
        </el-menu-item>
        <div style="flex-grow: 1"></div>
        <el-menu-item index="4">
          <span>设置</span>
        </el-menu-item>
      </el-menu>
    </div>

    <div class="flex-col-fill" style="margin-left: 8px;">
      <KeepAlive>
        <component :is="pages[index]"/>
      </KeepAlive>
    </div>

    <div id="window-actions">
      <el-button size="small" text @click="appWindow.minimize()">
        <icon size="18">
          <subtract24-regular/>
        </icon>
      </el-button>
      <el-button size="small" text @click="appWindow.toggleMaximize()">
        <icon size="18">
          <maximize24-regular/>
        </icon>
      </el-button>
      <el-button size="small" text @click="appWindow.close()">
        <icon size="18">
          <dismiss24-regular/>
        </icon>
      </el-button>
    </div>
  </div>
</template>

<script setup>
import {ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";
import {Dismiss24Regular, Maximize24Regular, Subtract24Regular} from "@vicons/fluent"
import {Icon} from "@vicons/utils";
import Classes from "./views/Classes.vue"
import Templates from "./views/Templates.vue";
import ClassColumns from "./views/ClassColumns.vue"
import PropColumns from "./views/PropColumns.vue";
import Setting from "./views/Setting.vue";

const index = ref(0)
const pages = [
  Classes,
  Templates,
  ClassColumns,
  PropColumns,
  Setting,
]

</script>

<style lang="scss" scoped>
.logo {
  height: 70px;
  width: auto;
  padding: 20px 0;
  will-change: filter;
  transition: 0.75s;
}

#title {
  width: 100%;
  height: 52px;
}

#window-actions {
  position: absolute;
  right: 0;
  top: 0;
  padding: 2px;
  z-index: 999;

  .el-button + .el-button {
    margin-left: 0;
  }
}

.el-menu {
  display: flex;
  flex-direction: column;
  border: none;
  padding-bottom: 8px;

  .el-menu-item {
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
</style>
