<template>
  <div class="flex-col-fill">
    <content-title>
      <el-button type="primary" :icon="Back" text style="margin-left: 4px;font-weight: bold" @click="onBack">返回
      </el-button>
      <el-button v-if="!!fileName" text style="margin-left: 4px" @click="onCopyName" >{{ fileName }}</el-button>
      <el-button v-if="!!fileName" text style="margin-left: 4px" @click="onCopyCode" >复制代码
      </el-button>
    </content-title>
    <div class="flex-row-fill">
      <div class="flex-col-fill" style="width: 100px;flex: none">
        <size-wrapper v-model:height="height">
          <el-table
              ref="tableRef"
              :height="height"
              :data="templates"
              highlight-current-row
              @current-change="m => selTem = m"
              border
          >
            <el-table-column prop="name" label="模板名"/>
          </el-table>
        </size-wrapper>
      </div>
      <code-editor v-model="preview" :mode="lanMode" read-only style="border-top: 1px solid var(--el-border-color-lighter)"/>
    </div>
  </div>
</template>

<script setup>
import {Back, Check} from "@element-plus/icons-vue";
import ContentTitle from "../components/ContentTitle.vue";
import CodeEditor from "../components/CodeEditor.vue";
import {computed, onMounted, ref, watch} from "vue";
import {Store} from "../api/store.js";
import {initRender, render} from "../api/template.js";
import SizeWrapper from "../components/SizeWrapper.vue";
import {camelCase, CamelCase, snake_case} from "../api/nameCase.js";
import {writeText} from "@tauri-apps/api/clipboard";
import {ElMessage} from "element-plus";

const prop = defineProps(['clazz'])
const emit = defineEmits(['back'])

const height = ref(0)
const tableRef = ref()
const templates = ref([])
const selTem = ref()
const clazzProps = ref([])
const lanMode = ref()
const preview = ref(null)

const languages = [
  {name: "Java", mode: "text/x-java"},
  {name: "Javascript", mode: "text/javascript"},
  {name: "Vue", mode: "text/x-vue"},
  {name: "Xml", mode: "application/xml"},
  {name: "Mysql", mode: "text/x-mysql"},
  {name: "Mssql", mode: "text/x-mssql"},
]

initRender()

const selChange = computed(() => {
  if (templates.value.length === 0 || !selTem.value || !clazzProps.value) return null
  return {
    clazz: prop.clazz,
    props: clazzProps.value
  }
})

watch(selChange, (n) => {
  if (!n) return
  Store.Templates.info(selTem.value.id).then(value => {
    lanMode.value = languages.find(l => l.name === selTem.value.language).mode
    preview.value = render(value, n)
  })
})

const fileName = computed(() => {
  if (!selTem.value) return null
  let name = selTem.value.fileName
  name = name.replace("{ClassName}",CamelCase(prop.clazz.name))
  name = name.replace("{className}",camelCase(prop.clazz.name))
  name = name.replace("{class_name}",snake_case(prop.clazz.name))
  return name
})

function onCopyName(){
  writeText(fileName.value)
  ElMessage({
    message: '复制成功',
    type: 'success',
  })
}

function onCopyCode() {
  writeText(preview.value)
  ElMessage({
    message: '复制成功',
    type: 'success',
  })
}

onMounted(() => {
  Store.Templates.list().then(value => {
    templates.value = value
    tableRef.value.setCurrentRow(value[0])
  })
  Store.Classes.info(prop.clazz.id).then(value => {
    clazzProps.value = value
  })
})


function onBack() {
  emit('back')
}
</script>

<style scoped lang="scss">

</style>