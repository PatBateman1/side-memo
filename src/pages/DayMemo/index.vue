<template>
  <div class="memo-container">
    <div class="title-container item-container">
      <div>{{ memoData.date }}</div>
      <img
        class="plus-icon"
        src="../../assets/plus_icon.png"
        @click="addMemo"
      />
    </div>

    <div class="content-container">
      <div
        v-if="showMemoInput"
        class="memo-input-container item-container"
      >
        <textarea
          class="memo-input"
          v-model="newMemoContent"
        ></textarea>
        <img
          class="check-icon"
          src="../../assets/check_icon.png"
          @click="submit"
        />
      </div>

      <div class="subtitle">待完成</div>
  
      <div
        v-for="(item, index) in memoData.list"
        :key="index"
        class="unfinished-list item-container"
        :style="`display: ${item.complete ? 'none': ''}`"
      >
        <div>{{ item.name }}</div>
        <div class="tools-panel">
          <img
            class="tool-icon"
            src="../../assets/trash_icon.png"
            @click="chengeMemoItemStatus(index, 0)"
          />
          <img
            class="tool-icon"
            src="../../assets/check_icon.png"
            @click="chengeMemoItemStatus(index, 1)"
          />
        </div>
      </div>
    
      <div class="subtitle">已完成</div>
      <div
        v-for="(item, index) in memoData.list"
        :key="index"
        class="finished-list item-container"
        :style="`display: ${!item.complete ? 'none': ''}`"
      >
        <div>{{ item.name }}</div>
        <div class="tools-panel">
          <img
            class="tool-icon"
            src="../../assets/up_left_icon.png"
            @click="chengeMemoItemStatus(index, 1)"
          />
          <img
            class="tool-icon"
            src="../../assets/trash_icon.png"
            @click="chengeMemoItemStatus(index, 0)"
          />
        </div>
      </div>
    </div>

  </div>
</template>


<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';

interface MemoItem {
  name: string;
  complete: boolean;
};

interface MemoData {
  date: string;
  list: MemoItem[];
};

enum OPERATE_MODE {
  DELETE = 0,
  CHANGE_COMPLETE = 1,
};

export default {
  data () {
    return {
      showMemoInput: false,
      newMemoContent: '',
      memoData: {} as MemoData,
    };
  },
  async created () {
    this.getContent();
  },
  methods: {
    async getContent () {
      const memoData: string = await invoke('get_memo_by_day', { offset: 0 });
      this.memoData = JSON.parse(memoData);
    },
    addMemo () {
      this.showMemoInput = true;
    },
    async submit () {
      this.memoData.list.unshift({
        name: this.newMemoContent,
        complete: false,
      });

      if (this.newMemoContent.length) {
        await invoke('set_memo', {
          date: this.memoData.date,
          content: JSON.stringify(this.memoData),
        });

        this.getContent();
      }


      this.showMemoInput = false
      this.newMemoContent = '';
    },
    async chengeMemoItemStatus (index: number, mode: number) {
      if (mode === OPERATE_MODE.DELETE) {
        this.memoData.list.splice(index, 1); 
      } else if (mode === OPERATE_MODE.CHANGE_COMPLETE) {
        const currItem = this.memoData.list[index];
        currItem.complete = !currItem.complete;
      }

      await invoke('set_memo', {
        date: this.memoData.date,
        content: JSON.stringify(this.memoData),
      });

      this.getContent();
    }
  }
};
</script>

<style lang="less" scoped>
.memo-container {
  .item-container {
    margin-bottom: 20px;
    padding: 20px;
    width: 100%;
    color: #f6f6f6;
    background-color: #2f2f2f;
    border-radius: 20px;
    box-sizing: border-box;
    overflow: hidden;
    &:hover {
      .tools-panel {
        display: block;
      }
    }
  }
  .title-container {
    position: fixed;
    justify-content: space-between;
    align-items: center;
    left: 0;
    top: 0;
    display: flex;
    width: 100%;
    font-size: 26px;
    font-weight: bold;
    .plus-icon {
      height: 28px;
      widows: 28px;
    }
  }
  .content-container {
    margin-top: 90px;
  }
  .subtitle {
    margin-bottom: 10px;
    font-size: 20px;
    font-weight: bold;
  }
  .memo-input-container {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    box-sizing: border-box;
    .check-icon {
      position: absolute;
      bottom: 25px;
      right: 30px;
      width: 40px;
      height: 40px;
    }
  }
  .memo-input {
    padding: 15px 15px 40px 15px;
    height: 150px;
    width: 95% !important;
    font-size: 18px;
    color: #f6f6f6;
    background-color: #2f2f2f;
    border-radius: 8px;
  }
  .unfinished-list {
    font-size: 18px;
    white-space: pre-line;
    word-break: break-all;
  }
  .finished-list {
    font-size: 18px;
    white-space: pre-line;
    word-break: break-all;
  }
  .tools-panel {
    margin-top: 10px;
    height: 30px;
    display: flex;
    align-items: center;
    display: none;
    .tool-icon {
      margin-right: 10px;
      width: 22px;
      height: 22px;
    }
  }
}
</style>
