<template>
  <svg
    class="circle-loader"
    width="40"
    height="40"
    version="1.1"
    xmlns="http://www.w3.org/2000/svg"
  >
    <circle cx="20" cy="20" r="15" />
  </svg>
</template>

<style lang="less" scoped>
@import 'node_modules/nord/src/lesscss/nord.less';

.circle-loader {
  margin-left: 48%;
  fill: transparent;
  stroke: @nord7;
  stroke-width: 5;
  animation: dash 1.5s ease infinite, rotate 2s linear infinite;
}

@keyframes dash {
  0% {
    stroke-dasharray: 1, 95;
    stroke-dashoffset: 0;
  }
  50% {
    stroke-dasharray: 85, 95;
    stroke-dashoffset: -25;
  }
  100% {
    stroke-dasharray: 85, 95;
    stroke-dashoffset: -90;
  }
}

@keyframes rotate {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
