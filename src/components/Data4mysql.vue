<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { open, message } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

const getDataMsg = ref("");
const getYamlMsg = ref("");
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
    await message("未选择yaml文件", "提示");
    return;
  }

  await invoke("download", {
    filePath: data.filePath,
  });
  getDataMsg.value = "Downloading..."
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

</script>

<template>
  <div class="card">
    <button type="button" @click="selectFile()">Open Yaml</button>
    <p>{{ getYamlMsg }}</p>

    <p></p>
    <button type="button" @click="getData()">Download</button>
  </div>

  <p>{{ getDataMsg }}</p>
</template>
