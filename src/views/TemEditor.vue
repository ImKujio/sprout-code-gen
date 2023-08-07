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
    <el-dialog v-model="help" title="模板语法" width="60%">
      <el-scrollbar height="300px">
        <section>
          <h4>输出</h4>
          <pre><code>{{ doc.outPut }}</code></pre>
          <h4>原文输出</h4>
          <pre><code>{{ doc.outputOrg }}</code></pre>
          <h4>条件</h4>
          <pre><code>{{ doc.condition }}</code></pre>
          <h4>循环</h4>
          <pre><code>{{ doc.circulation }}</code></pre>
          <h4>循环</h4>
          <pre><code>{{ doc.circulation }}</code></pre>
          <h4>命名转换</h4>
          <pre><code>{{ doc.nameCase }}</code></pre>
          <h4>行折叠</h4>
          <pre><code>{{ doc.inline }}</code></pre>
        </section>
      </el-scrollbar>
    </el-dialog>
  </div>
</template>

<script setup>
import ContentTitle from "../components/ContentTitle.vue"
import {Back, Refresh, Warning} from "@element-plus/icons-vue";
import {Store} from "../api/store.js";
import {ref, watch} from "vue";
import CodeEditor from "../components/CodeEditor.vue";
import {initRender, render} from "../api/template.js";
import doc from "../api/doc.js";

const prop = defineProps(['temId', 'mode', 'temName'])
const emit = defineEmits(['back', 'edited'])
const preMod = ref(null)
const preModInfo = ref(null)
const modules = ref([])
const code = ref(null)
const preview = ref(null)
const help = ref(false)

Store.Classes.list().then(value => {
  if (value != null) modules.value = value
})

Store.Templates.info(prop.temId).then(value => {
  code.value = value
}).finally(() => {
  watch(code, async n => {
    await Store.Templates.setInfo(prop.temId, n)
  })
})

watch(preMod, async n => {
  if (n != null) preModInfo.value = await Store.Classes.info(n.id)
})

initRender()

function onRefresh() {
  preview.value = render(code.value, {
    clazz: preMod.value,
    props: preModInfo.value
  })
}

function onHelp() {
  help.value = true
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