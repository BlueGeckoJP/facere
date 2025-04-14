<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/core";
import { onMounted, onUnmounted, ref } from "vue";
import AddTodo from "./components/AddTodo.vue";
import TodoItem from "./components/TodoItem.vue";

type SqlTodo = {
  uuid: string;
  title: string;
  checked: boolean;
  deadline: string;
};

type TodoState = {
  title: string;
};

type UUID = string;

const todoList = ref(new Map<UUID, TodoState>());
const completedTodoList = ref(new Map<UUID, TodoState>());

const nowDate = ref(
  new Intl.DateTimeFormat("en-GB", {
    timeZone: "Asia/Tokyo",
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    weekday: "short",
  }).format(new Date())
);

let dateInterval: number;

function updateTodos() {
  invoke("get_todos")
    .then((t) => {
      let todos = t as [SqlTodo];

      todoList.value.clear();
      completedTodoList.value.clear();

      todos.forEach((todo) => {
        if (todo.checked) {
          completedTodoList.value.set(todo.uuid, todo);
        } else {
          todoList.value.set(todo.uuid, todo);
        }
      });
    })
    .catch((e) => alert(e));
}

function updateCount() {
  let count = todoList.value.size + completedTodoList.value.size;
  let completedCount = completedTodoList.value.size;
  return `${completedCount} / ${count} Task Completed`;
}

function onEmitChecked(uuid: UUID) {
  invoke("check_todo", { uuid: uuid }).catch((e) => alert(e));
  updateTodos();
}

onMounted(() => {
  updateTodos();

  dateInterval = setInterval(() => {
    nowDate.value = new Intl.DateTimeFormat("en-GB", {
      timeZone: "Asia/Tokyo",
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      weekday: "short",
    }).format(new Date());
  }, 60000); // 60 * 1000 = 1min
});

onUnmounted(() => {
  clearInterval(dateInterval);
});
</script>

<template>
  <div id="app">
    <div id="title-container">
      <p id="today-title">Today's ToDo</p>
      <div id="subtitle-container">
        <p>{{ nowDate }}</p>
        <p>{{ updateCount() }}</p>
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
      <AddTodo :updateTodos="updateTodos" />
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
