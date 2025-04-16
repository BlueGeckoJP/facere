<script lang="ts" setup>
import { faTrash } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { invoke } from "@tauri-apps/api/core";
import { SqlTodo } from "../types";

const props = defineProps({
  todo: {
    type: Object as () => SqlTodo,
    required: true,
  },
  updateTodos: {
    type: Function,
    required: true,
  },
});
const checkedEmit = defineEmits(["checked"]);
const todo = props.todo;

function onClickCheckbox(_: MouseEvent) {
  checkedEmit("checked", todo.uuid);
}

function onClickTrash(_: MouseEvent) {
  confirm("Are you sure you want to delete it?") &&
    invoke("delete_todo", { uuid: todo.uuid }).catch((e) => alert(e));

  props.updateTodos();
}
</script>

<template>
  <div class="todo-item">
    <div class="title-container">
      <a
        class="checkbox"
        :class="{ checked: todo.checked }"
        @click="onClickCheckbox"
      ></a>
      <a class="trash" @click="onClickTrash">
        <FontAwesomeIcon :icon="faTrash" />
      </a>
      <span>{{ todo.title }}</span>
    </div>
    <div>
      <span>{{ todo.deadline }}</span>
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
