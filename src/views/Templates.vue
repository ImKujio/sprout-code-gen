<template>
  <div class="flex-col-fill">
    <div v-if="!editPage" class="flex-col-fill">
      <content-title>
        <div v-if="!editPage">
          <el-button type="primary" :icon="Plus" text style="margin-left: 4px;font-weight: bold" @click="onAdd">新增
          </el-button>
          <el-button text style="margin-left: 4px" @click="onEdit" :disabled="!treeSel">编辑
          </el-button>
          <el-button text style="margin-left: 4px" @click="onTemEdit" :disabled="!treeSel || treeSel.group === 1">编辑模板
          </el-button>
          <el-button type="danger" text style="margin-left: 4px" @click="onDel"
                     :disabled="!treeSel || (treeSel.child && treeSel.child.length > 0)">删除
          </el-button>
        </div>
        <div v-else>
          <el-button type="primary" :icon="Back" text style="margin-left: 4px;font-weight: bold"
                     @click="onBack">返回
          </el-button>
        </div>
      </content-title>

      <size-wrapper v-if="!editPage" v-model:height="height">
        <el-table
            :max-height="height"
            :data="temTree"
            row-key="id"
            highlight-current-row
            @current-change="t => treeSel = t"
            default-expand-all
            :tree-props="{ children: 'child' }"
        >
          <el-table-column label="模板名" width="250">
            <template #default="scope">
              <span>{{ getTemById(scope.row.id).name }}</span>
            </template>
          </el-table-column>
          <el-table-column label="文件名">
            <template #default="scope">
              <span>{{ getTemById(scope.row.id).fileName }}</span>
            </template>
          </el-table-column>
          <el-table-column label="语言" width="100">
            <template #default="scope">
              <span>{{ getTemById(scope.row.id).language }}</span>
            </template>
          </el-table-column>
          <el-table-column label="更新时间" width="150">
            <template #default="scope">
              <span>{{ getTemById(scope.row.id).update }}</span>
            </template>
          </el-table-column>
        </el-table>
      </size-wrapper>

      <TemEditor v-else :template="selTem.id"/>

      <el-dialog v-model="dialog" title="添加模板" width="600">
        <el-form ref="formRef" :model="form" label-width="auto">
          <el-row :gutter="12">
            <el-col :span="12">
              <el-form-item prop="group" label="类型">
                <el-radio-group v-model="form.group" :disabled="form.id">
                  <el-radio :label="0">模板</el-radio>
                  <el-radio :label="1">目录</el-radio>
                </el-radio-group>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item prop="pid" label="父目录">
                <el-select v-model="form.pid">
                  <el-option :value="0" label="主目录"/>
                  <el-option
                      v-for="item in templates.filter(t => t.group === 1)"
                      :value="item.id"
                      :label="item.name"
                      :key="item.id"/>
                </el-select>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item prop="name" label="模板名" :rules="[
              {required: true, message: '请输入模板名', trigger: 'blur'}
             ]">
                <el-input v-model="form.name" clearable placeholder="请输入模板名"/>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item prop="fileName" :rules="[
              {required: true, message: '请输入文件名', trigger: 'blur'},
             ]">
                <template #label>
                  <span>
                    <el-tooltip content="文件名可插入类名,大驼峰:{ClassName},小驼峰:{className},蛇形:{class_name}"
                                placement="top">
                    <icon style="vertical-align: middle">
                      <el-icon>
                        <QuestionFilled/>
                      </el-icon>
                    </icon>
                  </el-tooltip>
                  </span>
                  文件名
                </template>
                <el-input v-model="form.fileName" clearable placeholder="请输入类名称"/>
              </el-form-item>
            </el-col>
            <el-col :span="12" v-if="form.group===0">
              <el-form-item prop="language" label="语言" :rules="[
              {required: true, message: '请选择语言', trigger: 'blur'},
             ]">
                <el-select v-model="form.language" placeholder="请输选择语言">
                  <el-option
                      v-for="(item,index) in languages"
                      :value="item.name"
                      :label="item.name"
                      :key="index"/>
                </el-select>
              </el-form-item>
            </el-col>
          </el-row>
        </el-form>
        <template #footer>
      <span class="dialog-footer">
        <el-button @click="dialog = false">取消</el-button>
        <el-button type="primary" @click="onSubmit">
          确定
        </el-button>
      </span>
        </template>
      </el-dialog>
    </div>
    <tem-editor v-else :tem-id="selTem.id" :tem-name="selTem.name" :mode="lanMode" @back="onBack"
                @edited="onCodeEdited"/>
  </div>
</template>

<script setup>
import {computed, reactive, ref, watch} from "vue";
import SizeWrapper from "../components/SizeWrapper.vue";
import ContentTitle from "../components/ContentTitle.vue";
import {Back, Plus, QuestionFilled} from "@element-plus/icons-vue";
import {Store} from "../api/store.js";
import {array2Tree} from "../tools/array.js";
import TemEditor from "./TemEditor.vue";
import {ElMessageBox} from "element-plus";

const height = ref(0)

const templates = reactive([])
const dialog = ref(false)
const form = reactive({})
const formRef = ref(null)
const treeSel = ref(null)
const selTem = ref(null)
const lanMode = ref(null)
const editPage = ref(false)

const languages = [
  {name: "Java", mode: "text/x-java"},
  {name: "Javascript", mode: "text/javascript"},
  {name: "Vue", mode: "text/x-vue"},
  {name: "Xml", mode: "application/xml"},
  {name: "Mysql", mode: "text/x-mysql"},
  {name: "Mssql", mode: "text/x-mssql"},
]

function initForm(val) {
  form.id = !val ? null : val.id
  form.pid = !val ? 0 : val.pid
  form.group = !val ? 1 : val.group
  form.name = !val ? null : val.name
  form.fileName = !val ? null : val.fileName
  form.language = !val ? null : val.language
  form.update = !val ? null : val.update
}

const temTree = computed(() => {
  return array2Tree(templates.map(t => {
    return {
      id: t.id,
      pid: t.pid,
      group: t.group
    }
  }), 0)
})

watch(treeSel, n => {
  if (n == null) {
    selTem.value = null
    return
  }
  selTem.value = templates.find(t => n.id === t.id)
})

Store.Templates.list().then(res => {
  if (res != null) templates.push(...res)
}).finally(() => {
  watch(templates, async n => {
    await Store.Templates.setList(n)
  })
})

function getTemById(id) {
  return templates.find(t => t.id === id)
}

async function onSubmit() {
  await formRef.value.validate(valid => {
    if (!valid) return
    const now = new Date()
    form.update = now.format("yy-MM-dd hh:mm:ss")
    if (form.group === 1) form.language = null
    if (form.id) {
      const index = templates.findIndex(t => t.id === form.id)
      templates.splice(index, 1, Object.assign({}, form))
    } else {
      form.id = now.getTime()
      templates.push(Object.assign({}, form))
    }
    dialog.value = false
  })
}

function onBack() {
  editPage.value = false
  treeSel.value = null
}

function onAdd() {
  initForm()
  if (selTem.value != null)
    form.pid = selTem.value.group === 1 ? selTem.value.id : selTem.value.pid
  dialog.value = true
}

function onEdit() {
  initForm(selTem.value)
  dialog.value = true
}

function onCodeEdited() {
}

function onTemEdit() {
  lanMode.value = languages.find(l => l.name === selTem.value.language).mode
  editPage.value = true
}

async function onDel() {
  await ElMessageBox.confirm(`是否确认删除模板：${selTem.value.name}`,'警告',{type: 'warning'})
  templates.splice(templates.indexOf(selTem.value), 1)
}
</script>

<style lang="scss" scoped>
</style>