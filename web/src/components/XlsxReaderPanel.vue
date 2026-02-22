<script setup lang="ts">
import { ref } from "vue";
import { read_xlsx } from "../xlsx-reader";
import SheetTableList from "./SheetTableList.vue";

type CellValue = string | number | boolean | undefined;

type SheetData = {
  headers: string[] | null;
  rows: CellValue[][];
};

const sheets = ref<Array<{ name: string; data: SheetData }>>([]);
const elapsedMs = ref<number | null>(null);
const errorMessage = ref("");

async function onFileChange(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;

  errorMessage.value = "";
  sheets.value = [];
  elapsedMs.value = null;

  try {
    const buffer = await file.arrayBuffer();
    const t0 = performance.now();
    const info = read_xlsx(new Uint8Array(buffer));
    const t1 = performance.now();

    sheets.value = Array.from(info.entries()).map(([name, data]) => ({ name, data }));
    elapsedMs.value = t1 - t0;

    console.log(
      "table_names:",
      sheets.value.map((sheet) => sheet.name),
    );
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
      <p class="subtitle">选择一个 .xlsx 文件，读取并展示每个 Sheet 的表头和数据。</p>

      <label class="picker">
        <span class="picker-text">选择 .xlsx 文件</span>
        <input type="file" accept=".xlsx" @change="onFileChange" />
      </label>

      <p v-if="elapsedMs !== null" class="elapsed">用时: {{ elapsedMs.toFixed(2) }}ms</p>
      <SheetTableList v-if="sheets.length" :sheets="sheets" />

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

.error {
  margin-top: 14px;
  color: #b91c1c;
  font-weight: 600;
}
</style>
