<script setup lang="ts">
import { ref } from "vue";

// 用于显示返回的消息
const pingResponse = ref("");
const addResponse = ref("");
const addPostResponse = ref("");

// 测试 /ping 路由
async function testPing() {
  try {
    const response = await fetch("http://localhost:19801/ping");
    pingResponse.value = await response.json();
  } catch (error) {
    pingResponse.value = `Error: ${error}`;
  }
}

// 测试 /add/{x}/{y} 路由
async function testAdd(x: number, y: number) {
  try {
    const response = await fetch(`http://localhost:19801/add/${x}/${y}`);
    addResponse.value = await response.text();
  } catch (error) {
    addResponse.value = `Error: ${error}`;
  }
}

// 测试 POST /add_post 路由
async function testAddPost(x: number, y: number) {
  try {
    const response = await fetch("http://localhost:19801/add_post", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ x, y }),
    });
    addPostResponse.value = await response.text();
  } catch (error) {
    addPostResponse.value = `Error: ${error}`;
  }
}
</script>

<template>
  <div>
    <h2>测试 Julia 服务</h2>

    <!-- Ping 测试 -->
    <button @click="testPing">Ping 测试</button>
    <p>Ping 结果: {{ pingResponse }}</p>

    <!-- Add 测试 -->
    <button @click="testAdd(3.5, 4.5)">GET Add (3.5 + 4.5)</button>
    <p>Add 结果: {{ addResponse }}</p>

    <!-- Add POST 测试 -->
    <button @click="testAddPost(3.5, 4.5)">POST Add (3.5 + 4.5)</button>
    <p>POST Add 结果: {{ addPostResponse }}</p>
  </div>
</template>

<style scoped>
button {
  padding: 0.5em 1em;
  margin: 0.5em;
  border-radius: 8px;
  background-color: #396cd8;
  color: white;
  border: none;
  cursor: pointer;
}

button:hover {
  background-color: #355db2;
}

p {
  font-family: Arial, sans-serif;
  font-size: 1em;
  color: #333;
}
</style>
