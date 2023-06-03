<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import Notification from './Notification.vue'

const getDataMsg = ref("");
const getYamlMsg = ref("");
const notification = ref();
const data = reactive({
  filePath: "",
  fileFormats: ["yaml"],
});

onMounted(() => {
  setTimeout(() => {
    invoke("close_splashscreen");
  }, 2000);
});

// download mysql data
async function getData() {
  if (data.filePath == "") {
    notification.value.warn("未选择yaml文件")
    return;
  } 

  if (data.filePath != "") {
    getDataMsg.value = "正在下载,请稍等..."
    getDataMsg.value = await invoke("download", {
      filePath: data.filePath,
    });

    notification.value.success(getDataMsg.value)
  }
}

async function selectFile() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: 'Yaml',
        extensions: data.fileFormats,
      },
    ]
  });
  if (Array.isArray(selected)) {
    // user selected multiple files
    data.filePath = selected.toString();
  } else if (selected === null) {
    // user cancelled the selection
    return;
  } else {
    // user selected a single file
    data.filePath = selected;
  }
  getYamlMsg.value = selected.toString();
}

function onClose() { // 点击默认关闭按钮时触发的回调函数
  console.log('关闭notification')
}

</script>

<template>
  <div class="card">
    <button type="button" @click="selectFile()">Open Yaml</button>
    <p>{{ getYamlMsg }}</p>

    <p></p>
    <button type="button" @click="getData()">Download</button>
  </div>

  <p>{{ getDataMsg }}</p>

  <Notification
        ref="notification"
        placement="topRight"
        :duration="5000"
        :top="20"
        @close="onClose" />
</template>
