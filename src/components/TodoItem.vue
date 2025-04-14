<script lang="ts" setup>
import { faTrash } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps({
  uuid: {
    type: String,
    required: true,
  },
  title: {
    type: String,
    required: true,
  },
  checked: {
    type: Boolean,
    required: true,
  },
  updateTodos: {
    type: Function,
    required: true,
  },
});
const checkedEmit = defineEmits(["checked"]);

function onClickCheckbox(_: MouseEvent) {
  checkedEmit("checked", props.uuid);
}

function onClickTrash(_: MouseEvent) {
  confirm("Are you sure you want to delete it?") &&
    invoke("delete_todo", { uuid: props.uuid }).catch((e) => alert(e));

  props.updateTodos();
}
</script>

<template>
  <div class="todo-item">
    <div class="title-container">
      <a
        class="checkbox"
        :class="{ checked: props.checked }"
        @click="onClickCheckbox"
      ></a>
      <a class="trash" @click="onClickTrash">
        <FontAwesomeIcon :icon="faTrash" />
      </a>
      <span>{{ props.title }}</span>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.title-container {
  display: flex;
  align-items: center;
}

.checkbox {
  display: inline-block;
  width: 1rem;
  height: 1rem;
  border: 1px solid orange;
  border-radius: 50%;
  position: relative;
  margin-right: 0.5rem;
}

.checked {
  &::after {
    position: absolute;
    left: 0.3rem;
    content: "";
    width: 0.3rem;
    height: 0.6rem;
    border-right: 1px solid orange;
    border-bottom: 1px solid orange;
    transform: rotate(45deg);
  }
}

.trash {
  margin-right: 0.5rem;

  svg {
    color: orange;
  }
}
</style>
