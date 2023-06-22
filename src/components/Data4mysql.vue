<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import Notification from './Notification.vue';
import { listen } from '@tauri-apps/api/event';

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
  }, 1000);
});

listen('progress', (event) => {
  const progress = event.payload as number;
  const progressBar = document.getElementById('progress-bar') as HTMLProgressElement;
  progressBar.value = progress as number;
  console.log(progressBar.value);
})

listen('message', (event) => {
  const progress = event.payload;
  notification.value.success(progress)
})

// download mysql data
async function getData() {
  if (data.filePath == "") {
    notification.value.warn("未选择yaml文件")
    return;
  } 

  if (data.filePath != "") {
    getDataMsg.value = "正在下载,请稍等..."
    let value = await invoke("download", { filePath: data.filePath})
      .then((msg) => notification.value.success(msg))
      .catch((err) => notification.value.error(err))
    getDataMsg.value = "***程序运行结束***"
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

  <progress id="progress-bar" max="100" value="0"></progress>

  <Notification
        ref="notification"
        placement="topRight"
        :duration="5000"
        :top="20"
        @close="onClose" />
</template>

<style>
  #progress-bar {
    width: 180px;
    height: 20px;
    margin: 0 auto;
    animation: pg 2s infinite linear;
  }
  @keyframes pg {
      100%{ background-size: 100%; }
  }
</style>