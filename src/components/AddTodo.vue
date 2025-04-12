<script setup lang="ts"></script>

<template>
  <div id="add-todo">
    <button>Add</button>
    <input type="text" placeholder="Title" />
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
