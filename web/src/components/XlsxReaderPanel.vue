<script setup lang="ts">
import { ref } from "vue";
import { read_xlsx } from "../xlsx-reader";

const tableNames = ref<string[]>([]);
const elapsedMs = ref<number | null>(null);
const errorMessage = ref("");

async function onFileChange(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;

  errorMessage.value = "";
  tableNames.value = [];
  elapsedMs.value = null;

  try {
    const buffer = await file.arrayBuffer();
    const t0 = performance.now();
    const info = read_xlsx(new Uint8Array(buffer));
    const t1 = performance.now();

    tableNames.value = info.table_names;
    elapsedMs.value = t1 - t0;

    console.log("table_names:", info.table_names);
    console.log(`read_xlsx 用时: ${elapsedMs.value.toFixed(2)}ms`);
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : String(error);
    console.error("读取 xlsx 失败:", error);
  } finally {
    input.value = "";
  }
}
</script>

<template>
  <main class="page">
    <section class="panel">
      <h1 class="title">XLSX Reader Demo</h1>
      <p class="subtitle">选择一个 .xlsx 文件，读取并输出 table_names 与耗时。</p>

      <label class="picker">
        <span class="picker-text">选择 .xlsx 文件</span>
        <input type="file" accept=".xlsx" @change="onFileChange" />
      </label>

      <p v-if="elapsedMs !== null" class="elapsed">用时: {{ elapsedMs.toFixed(2) }}ms</p>

      <section v-if="tableNames.length" class="result">
        <h2>table_names</h2>
        <ul>
          <li v-for="name in tableNames" :key="name">{{ name }}</li>
        </ul>
      </section>

      <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
    </section>
  </main>
</template>

<style scoped>
.page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.panel {
  width: min(720px, 100%);
  border-radius: 20px;
  padding: 28px;
  background: rgba(255, 255, 255, 0.9);
  border: 1px solid rgba(148, 163, 184, 0.25);
  box-shadow: 0 18px 40px rgba(15, 23, 42, 0.14);
  backdrop-filter: blur(4px);
}

.title {
  margin: 0;
  font-size: 30px;
  font-weight: 700;
  color: #0f172a;
}

.subtitle {
  margin: 10px 0 22px;
  color: #475569;
}

.picker {
  display: inline-flex;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  border-radius: 12px;
  background: #0f172a;
  color: #fff;
  cursor: pointer;
  transition: transform 0.15s ease, box-shadow 0.2s ease;
}

.picker:hover {
  transform: translateY(-1px);
  box-shadow: 0 8px 16px rgba(15, 23, 42, 0.28);
}

.picker input {
  width: 0;
  height: 0;
  opacity: 0;
  pointer-events: none;
}

.picker-text {
  font-weight: 600;
}

.elapsed {
  margin: 18px 0 0;
  font-weight: 600;
  color: #334155;
}

.result {
  margin-top: 18px;
  border-radius: 14px;
  padding: 14px 16px;
  background: #f8fafc;
  border: 1px solid #e2e8f0;
}

.result h2 {
  margin: 0 0 10px;
  font-size: 14px;
  letter-spacing: 0.03em;
  text-transform: uppercase;
  color: #475569;
}

.result ul {
  margin: 0;
  padding-left: 18px;
  color: #0f172a;
}

.result li + li {
  margin-top: 6px;
}

.error {
  margin-top: 14px;
  color: #b91c1c;
  font-weight: 600;
}
</style>
