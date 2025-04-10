<script setup lang="ts"></script>

<template>
  <div>
    <a id="add-todo-button">
      <div></div>
      <span>Add Todo</span>
    </a>
    <div>
      <input type="text" placeholder="Title" />
      <button>Add</button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
#add-todo-button {
  display: flex;
  align-items: center;

  div {
    display: inline-block;
    width: 1rem;
    height: 1rem;
    border: 1px solid navy;
    border-radius: 50%;
    position: relative;
    margin-right: 0.5rem;
  }
}
</style>
