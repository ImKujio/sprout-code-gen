<template>
  <div class="flex-col-fill">
    <content-title>
      <el-button type="primary" :icon="Back" text style="margin-left: 4px;font-weight: bold" @click="onBack">返回
      </el-button>
      <el-button text class="title">预览模块：
      </el-button>
      <el-select v-model="preMod" style="width: 150px" placeholder="选择模块以预览">
        <el-option
            v-for="(item,index) in modules"
            :value="item"
            :label="item.name"
            :key="index"/>
      </el-select>
      <el-button text :icon="Refresh" style="margin-left: 4px" @click="onRefresh" :disabled="!preMod">刷新预览
      </el-button>
      <el-button text :icon="Warning" style="margin-left: 4px" @click="onHelp">查看帮助
      </el-button>
    </content-title>
    <div class="flex-row-fill">
      <code-editor v-model="code" :mode="prop.mode"/>
      <code-editor v-model="preview" :mode="prop.mode" read-only/>
    </div>
  </div>
</template>

<script setup>
import ContentTitle from "../components/ContentTitle.vue"
import {Back, Refresh, Warning} from "@element-plus/icons-vue";
import {Data} from "../api/data.js";
import {ref, watch} from "vue";
import CodeEditor from "../components/CodeEditor.vue";
import {initRender, render} from "../api/template.js";

const prop = defineProps(['temId', 'mode', 'temName'])
const emit = defineEmits(['back', 'edited'])
const preMod = ref(null)
const preModInfo = ref(null)
const modules = ref(null)
const code = ref(null)
const preview = ref(null)

Data.Classes.get().then(value => {
  modules.value = value
})

Data.Templates.getInfo(prop.temId).then(value => {
  code.value = value
  watch(code, async n => {
    await Data.Templates.setInfo(prop.temId, n)
  })
})

watch(preMod, async n => {
  if (n == null) return
  preModInfo.value = await Data.Classes.getInfo(n.id)
})

initRender()
function onRefresh() {
  preview.value = render(code.value, {
    clazz: preMod.value,
    props: preModInfo.value
  })
}

function onHelp() {
}

function onBack() {
  emit('back')
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

</style>