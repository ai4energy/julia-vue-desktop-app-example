<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const juliaMsg = ref("");

// 启动 Julia 服务
async function startJulia() {
  try {
    juliaMsg.value = await invoke("start_julia_service");
  } catch (error) {
    juliaMsg.value = `Error: ${error}`;
  }
}

// 停止 Julia 服务
async function stopJulia() {
  try {
    juliaMsg.value = await invoke("stop_julia_service");
  } catch (error) {
    juliaMsg.value = `Error: ${error}`;
  }
}
</script>

<template>
  <div>
    <h2>Julia Service Control</h2>
    <div class="row">
      <button @click="startJulia">启动 Julia 服务</button>
      <button @click="stopJulia">停止 Julia 服务</button>
    </div>
    <p>{{ juliaMsg }}</p>
  </div>
</template>

<style scoped>
.row {
  display: flex;
  gap: 1em;
}
button {
  padding: 0.5em 1em;
  border: 1px solid #396cd8;
  background-color: #f6f6f6;
  cursor: pointer;
}
button:hover {
  background-color: #e8e8e8;
}
</style>
