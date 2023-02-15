<template>
  <div class="flex-col-fill">
    <content-title>
      <el-button type="primary" :icon="Back" text style="margin-left: 4px;font-weight: bold" @click="onBack">返回
      </el-button>
      <el-button type="primary" :icon="Plus" text style="margin-left: 4px;font-weight: bold" @click="onAdd">新增
      </el-button>
      <el-button text style="margin-left: 4px" @click="onEdit" :disabled="!selMod">编辑
      </el-button>
      <el-button type="danger" text style="margin-left: 4px" @click="onDel" :disabled="!selMod">删除
      </el-button>
    </content-title>
    <size-wrapper v-model:height="height">
      <el-table
          :max-height="height"
          :data="modProps"
          highlight-current-row
          @current-change="m => selMod = m"
      >
        <el-table-column type="index" align="center" label="序号" width="50"/>
        <el-table-column prop="name" label="属性"/>
        <el-table-column prop="comment" label="注释"/>
        <el-table-column v-for="(col,index) in columns" :key="index" :label="col.comment">
          <template #default="scope">
            <span v-if="col.type === '开关'"><el-icon v-if="scope.row[col.name] === 'true'"><Check/></el-icon></span>
            <span v-else>{{ scope.row[col.name] }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="update" label="更新时间" width="150"/>
      </el-table>
    </size-wrapper>
    <el-dialog v-model="dialog" title="添加模块" width="600">
      <el-form ref="formRef" :model="form" label-width="auto">
        <el-row :gutter="10">
          <el-col :span="12">
            <el-form-item prop="name" label="属性" :rules="[
              {required: true, message: '请输入属性名', trigger: 'blur'},
              {pattern: /^[a-z][A-Za-z]{2,20}$/, message: '格式错误,小驼峰格式,例:userName'}
            ]">
              <el-input v-model="form.name" clearable placeholder="请输入模块名称"/>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item prop="comment" label="注释" :rules="[
              {required: true, message: '请输入注释', trigger: 'blur'},
            ]">
              <el-input v-model="form.comment" clearable placeholder="请输入注释"/>
            </el-form-item>
          </el-col>
          <el-col v-for="col in columns" :span="12">
            <el-form-item :prop="col.name" :label="col.comment">
              <el-input v-if="col.type === '输入框'" v-model="form[col.name]" clearable
                        :placeholder="'请输入'+col.comment"/>
              <el-switch v-if="col.type === '开关'" v-model="form[col.name]" active-value="true"
                         inactive-value="false"/>
              <el-select v-if="col.type === '选择器'" v-model="form[col.name]" :placeholder="'请选择'+col.comment">
                <el-option v-for="(item,index) in col.options.split(',')"
                           :label="item"
                           :value="item"
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

</template>

<script setup>
import {reactive, ref, watch} from "vue";
import SizeWrapper from "../components/SizeWrapper.vue";
import ContentTitle from "../components/ContentTitle.vue";
import {Plus, Back,Check} from "@element-plus/icons-vue";
import {Store} from "../api/store.js";

const height = ref(0)

const prop = defineProps(['clazzId'])
const emit = defineEmits(['back', 'edited'])
const modProps = reactive([])
const columns = ref([])

const dialog = ref(false)
const form = reactive({})
const formRef = ref(null)
const selMod = ref(null)

function initForm(val) {
  form.id = !val ? null : val.id
  form.name = !val ? null : val.name
  form.comment = !val ? null : val.comment
  columns.value.forEach(col => {
    form[col.name] = !val ? col.defVal : val[col.name]
  })
  form.update = !val ? null : val.update
  console.log(form)
}

Store.Classes.info(prop.clazzId).then(res => {
  if (res != null) modProps.push(...res)
}).finally(() => {
  watch(modProps, async n => {
    await Store.Classes.setInfo(prop.clazzId, n)
  })
})

Store.PropColumns.list().then(value => {
  if (value != null) columns.value = value
})

async function onSubmit() {
  await formRef.value.validate(valid => {
    if (!valid) return
    const now = new Date()
    form.update = now.format("yy-MM-dd hh:mm:ss")
    if (form.id) {
      const index = modProps.findIndex(m => m.id = form.id)
      modProps.splice(index, 1, Object.assign({}, form))
    } else {
      form.id = now.getTime()
      modProps.push(Object.assign({}, form))
    }
    emit('edited', prop.clazzId)
    dialog.value = false
  })
}

function onBack() {
  emit('back')
}

function onAdd() {
  initForm()
  dialog.value = true
}

function onEdit() {
  initForm(selMod.value)
  dialog.value = true
}

function onDel() {
  modProps.splice(modProps.indexOf(selMod), 1)
  emit('edited', prop.modId)
}
</script>

<style lang="scss" scoped>
</style>