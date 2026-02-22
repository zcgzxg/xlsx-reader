<script setup lang="ts">
type CellValue = string | number | boolean | undefined;

type SheetData = {
  headers: string[] | null;
  rows: CellValue[][];
};

const props = defineProps<{
  sheets: Array<{
    name: string;
    data: SheetData;
  }>;
}>();

function getColumnCount(sheet: SheetData): number {
  const headerLen = sheet.headers?.length ?? 0;
  const rowMaxLen = sheet.rows.reduce((max, row) => Math.max(max, row.length), 0);
  return Math.max(headerLen, rowMaxLen);
}

function getHeaders(sheet: SheetData): string[] {
  const columnCount = getColumnCount(sheet);
  if (sheet.headers && sheet.headers.length > 0) {
    return Array.from({ length: columnCount }, (_, idx) => sheet.headers?.[idx] ?? `列${idx + 1}`);
  }

  return Array.from({ length: columnCount }, (_, idx) => `列${idx + 1}`);
}

function toDisplayValue(value: CellValue): string {
  return value === undefined ? "" : String(value);
}
</script>

<template>
  <section class="sheets">
    <article v-for="sheet in props.sheets" :key="sheet.name" class="sheet-card">
      <header class="sheet-header">
        <h2>{{ sheet.name }}</h2>
        <span>{{ sheet.data.rows.length }} 行</span>
      </header>

      <div class="sheet-table-wrap">
        <table class="sheet-table">
          <thead>
            <tr>
              <th v-for="header in getHeaders(sheet.data)" :key="header" :title="header">
                <span class="header-text">{{ header }}</span>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="sheet.data.rows.length === 0">
              <td :colspan="getHeaders(sheet.data).length || 1" class="empty-cell">无数据</td>
            </tr>
            <tr v-for="(row, rowIndex) in sheet.data.rows" :key="rowIndex">
              <td v-for="(_, colIndex) in getHeaders(sheet.data)" :key="colIndex">
                {{ toDisplayValue(row[colIndex]) }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </article>
  </section>
</template>

<style scoped>
.sheets {
  margin-top: 20px;
  display: grid;
  gap: 16px;
}

.sheet-card {
  border: 1px solid #e2e8f0;
  border-radius: 14px;
  background: #fff;
  overflow: hidden;
}

.sheet-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 14px;
  background: #f8fafc;
  border-bottom: 1px solid #e2e8f0;
}

.sheet-header h2 {
  margin: 0;
  font-size: 16px;
  color: #0f172a;
}

.sheet-header span {
  color: #64748b;
  font-size: 13px;
}

.sheet-table-wrap {
  overflow-x: auto;
}

.sheet-table {
  width: max-content;
  min-width: 100%;
  border-collapse: collapse;
  font-size: 14px;
  border: 1px solid #cbd5e1;
}

.sheet-table thead th {
  padding: 12px 14px;
  border: 1px solid #cbd5e1;
  background: #e2e8f0;
  text-align: left;
  color: #0f172a;
  font-weight: 700;
  font-size: 16px;
  line-height: 1.3;
  white-space: nowrap;
  max-width: 220px;
}

.header-text {
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sheet-table tbody td {
  padding: 10px 12px;
  border: 1px solid #cbd5e1;
  color: #1e293b;
  vertical-align: top;
  white-space: nowrap;
}

.sheet-table tbody tr:nth-child(even) {
  background: #fcfdff;
}

.empty-cell {
  text-align: center;
  color: #94a3b8;
}
</style>
