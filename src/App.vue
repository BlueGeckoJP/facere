<script lang="ts" setup>
import { ref } from "vue";
import TodoItem from "./components/TodoItem.vue";

// TODO: Use uuid for identification

const todoList = ref([1, 2, 3, 4, 5]);
const completedTodoList = ref([6, 7]);

// TODO: Rewrite this, it has a bug
function onEmitChecked(uuid: number, checked: boolean) {
  if (checked) {
    todoList.value.splice(todoList.value.indexOf(uuid), 1);
    completedTodoList.value.push(uuid);
  } else {
    completedTodoList.value.splice(completedTodoList.value.indexOf(uuid), 1);
    todoList.value.push(uuid);
  }
  console.log(todoList, completedTodoList);
}
</script>

<template>
  <div id="app">
    <div id="title-container">
      <p id="today-title">Today's ToDo</p>
      <div id="subtitle-container">
        <p>2025/01/01 Wed</p>
        <p>1 / 1 Task Completed</p>
      </div>
    </div>
    <div id="todo-container">
      <TodoItem
        v-for="todo in todoList"
        :key="todo"
        :uuid="todo"
        :checked="false"
        :title="'Test Item'"
        @checked="onEmitChecked"
      />
    </div>
    <div id="completed-todo-container">
      <TodoItem
        v-for="todo in completedTodoList"
        :key="todo"
        :uuid="todo"
        :checked="true"
        :title="'Test Completed Item'"
        @checked="onEmitChecked"
      />
    </div>
  </div>
</template>

<style lang="scss" scoped>
#title-container {
  margin: 1rem;
}

#subtitle-container {
  display: flex;
  p {
    margin-right: 1rem;
  }
}

#today-title {
  font-size: 2rem;
  font-weight: bold;
  margin-bottom: 0.5rem;
}

#todo-container,
#completed-todo-container {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin: 1rem;
}

#completed-todo-container {
  border-top: 1px solid grey;
  padding-top: 1rem;
  position: relative;
  margin-top: 2rem;

  &::before {
    content: "Completed";
    position: absolute;
    top: -0.7rem;
    left: 1rem;
    background: white;
    padding: 0 0.3rem;
  }
}
</style>
