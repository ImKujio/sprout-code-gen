<template>
  <size-wrapper v-model:height="height" v-model:width="width">
    <Codemirror
        v-model:value="prop.modelValue"
        :options="getOptions()"
        border
        placeholder="test placeholder"
        :height="height"
        :width="width"
        @change="onChange"
    />
  </size-wrapper>
</template>

<script setup>

import SizeWrapper from "./SizeWrapper.vue";
import Codemirror from "codemirror-editor-vue3";
// language
import "codemirror/mode/vue/vue"; //text/x-vue (Vue)
import "codemirror/mode/javascript/javascript" //text/javascript (Javascript)
import "codemirror/mode/sql/sql" //text/x-mysql (Mysql) text/x-mssql (Mssql)
import "codemirror/mode/clike/clike" //text/x-java (Java)
import "codemirror/mode/xml/xml" //application/xml (Xml)

// theme
import "codemirror/theme/idea.css";
// 自动补全
import "codemirror/addon/hint/show-hint.css";
import "codemirror/addon/hint/show-hint";
import "codemirror/addon/hint/anyword-hint";
// 自动括号匹配
import "codemirror/addon/edit/matchbrackets";
// 代码折叠
import "codemirror/addon/fold/foldgutter.css";
import "codemirror/addon/fold/foldcode.js";
import "codemirror/addon/fold/brace-fold.js";
import "codemirror/addon/fold/comment-fold.js";
import "codemirror/addon/fold/indent-fold.js";
import "codemirror/addon/fold/foldgutter.js";
// 光标行背景高亮
import "codemirror/addon/selection/active-line.js";
import {watch, ref, onMounted} from "vue";

const prop = defineProps({
  modelValue: String,
  mode: String,
  readOnly: {
    type: Boolean,
    default: false
  }
})
const emit = defineEmits(['update:modelValue'])
const height = ref(0)
const width = ref(0)

function onChange(val) {
  emit("update:modelValue", val)
}

watch(width, n => {
  console.log("code width:",n)
})

function getOptions() {
  if (!prop.readOnly) return { // 读写模式
    mode: prop.mode, // Language mode
    theme: "idea", // Theme
    lineNumbers: true, // Show line number
    smartIndent: true, // Smart indent
    indentUnit: 4, // The smart indent unit is 2 spaces in length
    foldGutter: true, // Code folding
    lineWrapping: false,
    gutters: ["CodeMirror-linenumbers", "CodeMirror-foldgutter", "CodeMirror-lint-markers"],
    styleActiveLine: true, // Display the style of the selected row
    matchBrackets: true,// 括号匹配
    extraKeys: {"Ctrl-Enter": "autocomplete"},//ctrl-space唤起智能提示
  }
  else return { // 只读模式
    mode: prop.mode, // Language mode
    theme: "idea", // Theme
    lineNumbers: true, // Show line number
    smartIndent: true, // Smart indent
    indentUnit: 4, // The smart indent unit is 2 spaces in length
    lineWrapping: false,
    gutters: ["CodeMirror-linenumbers", "CodeMirror-foldgutter"],
    styleActiveLine: true, // Display the style of the selected row
    readOnly: true
  }
}

</script>

<style scoped>

</style>