<template>
  <div class="flex-rwo-fill">
    <div class="flex-col-fill" style="width: auto;flex: none">
      <img src="/logo.svg" class="logo" alt="logo"/>
      <el-menu
          style="flex: 1"
          default-active="0">
        <el-menu-item index="0" @click="onMenuTap(0)">
          <span>模块列表</span>
        </el-menu-item>
        <el-menu-item index="1" @click="onMenuTap(1)">
          <span>Spring模板</span>
        </el-menu-item>
        <el-menu-item index="2" @click="onMenuTap(2)">
          <span>Page模板</span>
        </el-menu-item>
        <el-menu-item index="3" @click="onMenuTap(3)">
          <span>生成配置</span>
        </el-menu-item>
        <div style="flex-grow: 1"></div>
        <el-menu-item index="9" @click="onMenuTap(9)">
          <span>设置</span>
        </el-menu-item>
      </el-menu>
    </div>

    <div class="flex-col-fill" style="margin-left: 8px;">
      <div data-tauri-drag-region id="title">
        <el-tabs v-model="tabIndex" style="margin-left: 4px">
          <el-tab-pane v-for="(item,index) in tabs" :label="item" :name="index"/>
        </el-tabs>
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
      <div class="flex-col-fill" style="margin: 4px">
        <modules :tabindex="tabIndex" v-if="page === 0"/>
      </div>
    </div>
  </div>
</template>

<script setup>
import {reactive, ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";
import {Dismiss24Regular, Maximize24Regular, Subtract24Regular} from "@vicons/fluent"
import {Icon} from "@vicons/utils";
import Modules from "./views/Modules.vue"

const allTabs = [
  ['模块列表']
]

const page = ref(0)
const tabIndex = ref(0)
const tabs = ref(allTabs[0])

function onMenuTap(index) {
  page.value = index
  tabs.value = allTabs[index]
}
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
