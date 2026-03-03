<template>
  <div class="xlsx-reader">
    <button :loading="loading" @click="chooseXlsx">选择xlsx文件</button>

    <scroll-view scroll-x style="width: 100vw;">
      <rich-text :nodes="richNodes" />
    </scroll-view>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, shallowRef } from 'vue';
import { default as init, read_xlsx } from '../../xlsx-reader/lib.js';
import { onLoad } from '@dcloudio/uni-app';

type XlsxInfo = ReturnType<typeof read_xlsx>;
type ExtractValue<T> = T extends Map<string, infer V> ? V : never;
type XlsxItem = ExtractValue<XlsxInfo>;

const parseXlsxItemToRichText = (name: string, item: XlsxItem) => {
  return {
    name: 'table',
    children: [
      {
        name: 'caption',
        children: [
          {
            type: 'text',
            text: name,
          },
        ],
      },
      {
        name: 'thead',
        children:
          item.headers?.map((header) => ({
            name: 'td',
            attrs: {
              style: 'width: 36px;',
            },
            children: [
              {
                type: 'text',
                text: header,
              },
            ],
          })) ?? [],
      },
      {
        name: 'tbody',
        children:
          item.rows.map((row) => ({
            name: 'tr',
            children:
              row.map((cell) => ({
                name: 'td',
                children: [
                  {
                    type: 'text',
                    text: cell,
                  },
                ],
              })) ?? [],
          })) ?? [],
      },
    ],
  };
};
const loading = ref(true);
const xlsxInfo = shallowRef(null as XlsxInfo | null);
const richNodes = computed(() => {
  const info = xlsxInfo.value;

  if (!info) {
    return [
      {
        name: 'table',
        attrs: {
          style: 'width: 100%; height: 100%;',
        },
        children: [
          {
            name: 'caption',
            children: [
              {
                type: 'text',
                text: 'Loading...',
              },
            ],
          },
        ],
      },
    ];
  } else {
    const entries = Array.from(info.entries());
    const nodes = entries.map(([name, item]) => parseXlsxItemToRichText(name, item));
    return nodes;
  }
});

const chooseXlsx = () => {
  uni.chooseMessageFile({
    count: 1,
    type: 'file',
    extension: ['xlsx'],
    success: (res) => {
      if (res.tempFiles.length === 0) return;

      const now = Date.now();
      const file = res.tempFiles[0];
      const fm = uni.getFileSystemManager();
      const buffer = fm.readFileSync(file.path) as ArrayBuffer;
      const t1 = Date.now();

      try {
        xlsxInfo.value = read_xlsx(new Uint8Array(buffer));
      } catch (error) {
        console.error('read xlsx fail:', error);
      }

      const t2 = Date.now();
      console.log('read file time:', t1 - now, 'parse xlsx time:', t2 - t1);
    },
    fail: (err) => {
      console.error('chooseXlsx fail:', err);
    },
  });
};

onLoad(async () => {
  const now = Date.now();
  await init('/xlsx-reader/lib_bg.wasm.br');
  console.log('init time:', Date.now() - now);
  loading.value = false;
});
</script>

<style lang="less"></style>
