<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
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

updateTodos();

function onEmitChecked(uuid: UUID) {
  invoke("check_todo", { uuid: uuid }).catch((e) => alert(e));
  updateTodos();
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
