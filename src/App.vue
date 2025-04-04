<script lang="ts" setup>
import TodoItem from "./components/TodoItem.vue";
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
      <TodoItem />
      <TodoItem />
      <TodoItem />
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

#todo-container {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin: 1rem;
}
</style>
