<script lang="ts" setup>
import { ref } from "vue";
import TodoItem from "./components/TodoItem.vue";

type TodoState = {
  title: string;
};

// TODO: Use uuid for identification

const todoList = ref(new Map<number, TodoState>());
const completedTodoList = ref(new Map<number, TodoState>());

todoList.value.set(1, { title: "Test" });
todoList.value.set(2, { title: "Test2" });
completedTodoList.value.set(3, { title: "Test3" });

function onEmitChecked(uuid: number, checked: boolean) {
  console.log(uuid, checked);
  console.log(todoList, completedTodoList);
  if (!checked) {
    let todoItem = todoList.value.get(uuid);
    if (todoItem) {
      todoList.value.delete(uuid);
      completedTodoList.value.set(uuid, todoItem);
    }
    console.log(todoItem);
  } else {
    let todoItem = completedTodoList.value.get(uuid);
    if (todoItem) {
      completedTodoList.value.delete(uuid);
      todoList.value.set(uuid, todoItem);
    }
    console.log(todoItem);
  }
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
        :key="todo[0]"
        :uuid="todo[0]"
        :checked="false"
        :title="todo[1].title"
        @checked="onEmitChecked"
      />
    </div>
    <div id="completed-todo-container">
      <TodoItem
        v-for="todo in completedTodoList"
        :key="todo[0]"
        :uuid="todo[0]"
        :checked="true"
        :title="todo[1].title"
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
