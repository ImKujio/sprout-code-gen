<template>
  <div class="flex-col-fill">
    <content-title>
      <el-button text class="title">设置
      </el-button>
    </content-title>
    <size-wrapper v-model:height="height">
      <el-scrollbar :height="height">
        <el-card header="MySql" shadow="never">
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
                    连接
                  </el-button>
                </el-form-item>
              </el-col>
            </el-row>
          </el-form>
        </el-card>
      </el-scrollbar>
    </size-wrapper>
  </div>
</template>

<script setup>
import ContentTitle from "../components/ContentTitle.vue";
import SizeWrapper from "../components/SizeWrapper.vue";
import {Promotion, View, Hide} from "@element-plus/icons-vue";
import {reactive, ref, watch} from "vue";
import Database from "tauri-plugin-sql-api";
import {Store} from "../api/store.js";

const height = ref(0)
const setting = reactive({
  mysql: {}
})

Store.Setting.all().then(value => {
  if (value) for (let f in value) {
    setting[f] = value[f]
  }
}).finally(() => {
  watch(setting, async n => {
    await Store.Setting.put(n)
  })
})

async function onConnect() {
  const url = `mysql://${setting.mysql.user}:${setting.mysql.psw}@${setting.mysql.addr}/${setting.mysql.db}`
  console.log(url)
  const db = await Database.load(url)
  console.log("has connected")
  const rst = await db.execute("select 1")
  console.log(rst)
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