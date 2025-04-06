<script lang="ts" setup>
const props = defineProps(["uuid", "checked", "title"]);
const checkedEmit = defineEmits(["checked"]);

function onClickCheckbox(_: MouseEvent) {
  checkedEmit("checked", props.uuid, props.checked);
}
</script>

<template>
  <div class="todo-item">
    <div class="title-container">
      <a
        class="checkbox"
        :class="{ checked: props.checked }"
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
