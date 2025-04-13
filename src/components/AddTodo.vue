<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { v4 as uuidv4 } from "uuid";
import { ref } from "vue";

const props = defineProps({
  updateTodos: {
    type: Function,
    required: true,
  },
});

const buttonRef = ref();
const inputRef = ref();

function onClick() {
  let input = inputRef.value as HTMLInputElement;

  invoke("add_todo", {
    todo: {
      uuid: uuidv4(),
      title: input.value,
      checked: false,
      deadline: new Date().toISOString(),
    },
  })
    .then(() => {
      console.log("Added todo");
      input.value = "";
    })
    .catch((e) => {
      alert("Failed to add todo. Error: " + e);
    });

  props.updateTodos();
}
</script>

<template>
  <div id="add-todo">
    <button ref="buttonRef" @click="onClick">Add</button>
    <input type="text" placeholder="Title" ref="inputRef" />
  </div>
</template>

<style lang="scss" scoped>
#add-todo {
  padding: 0.5rem;
  border: 1px solid grey;
  border-radius: 15px;
  margin-right: auto;

  input {
    border: none;
    outline: none;
  }

  button {
    margin-right: 0.5rem;
  }
}
</style>
