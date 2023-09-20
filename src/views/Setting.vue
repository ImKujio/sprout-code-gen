<template>
  <div class="flex-col-fill">
    <content-title>
      <el-button text class="title">设置
      </el-button>
    </content-title>
    <size-wrapper v-model:height="height">
      <el-scrollbar :height="height">
        <el-card v-loading="mysqlLoading" header="MySql" shadow="never">
          <el-form label-width="auto">
            <el-row :gutter="40">
              <el-col :span="8">
                <el-form-item label="地址">
                  <el-input v-model="setting.mysql.addr" clearable></el-input>
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="数据库">
                  <el-input v-model="setting.mysql.db" clearable></el-input>
                </el-form-item>
              </el-col>
            </el-row>
            <el-row :gutter="40">
              <el-col :span="8">
                <el-form-item label="用户名">
                  <el-input v-model="setting.mysql.user" clearable></el-input>
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="密码">
                  <el-input v-model="setting.mysql.psw" type="password" clearable>
                  </el-input>
                </el-form-item>
              </el-col>
            </el-row>
            <el-row>
              <el-col>
                <el-form-item label=" ">
                  <el-button type="primary" :icon="Promotion" plain style="font-weight: bold" @click="onConnect"
                             :disabled="!setting.mysql.addr || !setting.mysql.db || !setting.mysql.user || !setting.mysql.psw">
                    测试连接
                  </el-button>
                </el-form-item>
              </el-col>
            </el-row>
          </el-form>
        </el-card>
        <el-card v-loading="mysqlLoading" header="数据" shadow="never">
          <el-button type="primary" :icon="FolderOpened" plain style="font-weight: bold" @click="onOpenDataDir">
            数据文件夹
          </el-button>
        </el-card>
      </el-scrollbar>
    </size-wrapper>
  </div>
</template>

<script setup>
import ContentTitle from "../components/ContentTitle.vue";
import SizeWrapper from "../components/SizeWrapper.vue";
import {Promotion, FolderOpened} from "@element-plus/icons-vue";
import {reactive, ref, watch} from "vue";
import {Store} from "../api/store.js";
import Mysql, {parseUrl} from "../api/mysql.js";
import {ElMessage} from "element-plus";
import { Command } from '@tauri-apps/api/shell'
import {dataDir} from '@tauri-apps/api/path';

const height = ref(0)
const mysqlLoading = ref(false)
const setting = reactive({
  mysql: {}
})

async function onOpenDataDir() {
  const dir = await dataDir()
  console.log(dir)
  const openCmd = new Command("showFile",[dir+'me.kujio.sprout.gen'])
  await openCmd.execute()
  // await open("\\" + dir, "explorer")
}

Store.Setting.all().then(value => {
  if (value) for (let f in value) {
    setting[f] = value[f]
  }
}).finally(() => {
  watch(setting, async n => {
    await Store.Setting.put(n)
  })
})

function onConnect() {
  mysqlLoading.value = true
  const url = parseUrl(setting.mysql.user, setting.mysql.psw, setting.mysql.addr, setting.mysql.db)
  Mysql.load(url).then(m => m.query("select version()"))
      .then(v => {
        console.log(v)
        ElMessage.success("连接成功,Mysql:" + v.rows[0][0])
      })
      .catch(e => ElMessage.error("连接失败:" + e))
      .finally(() => mysqlLoading.value = false)
}

</script>

<style lang="scss" scoped>
.title {
  padding-left: 8px;
  padding-right: 4px;
  margin-left: 0 !important;
  background-color: transparent !important;
  cursor: default !important;

}

.el-card {
  margin-right: 10px;
  margin-bottom: 10px;
}

.el-input-number {
  text-align: left;
  width: 100px;
  line-height: 1;
}
</style>