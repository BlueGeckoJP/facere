<script lang="ts" setup>
const props = defineProps(["uuid", "checked", "title"]);
const checkedEmit = defineEmits(["checked"]);

function onClickCheckbox(_: MouseEvent) {
  const checkbox = document.querySelector(".checkbox") as HTMLAnchorElement;
  checkedEmit("checked", props.uuid, checkbox.classList.contains("checked"));
}
</script>

<template>
  <div class="todo-item">
    <div class="title-container">
      <a
        class="checkbox {{ props.checked ? 'checked' : '' }}"
        @click="onClickCheckbox"
      ></a>
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
</style>
