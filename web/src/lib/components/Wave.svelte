<script lang="ts">
  const { class: className, fill }: { class: string; fill: string } = $props()
</script>

<svg
  class={className}
  viewBox="0 0 1440 320"
  preserveAspectRatio="none"
  xmlns="http://www.w3.org/2000/svg"
>
  <path {fill} d="M0,160 C240,220 480,80 720,160 C960,240 1200,80 1440,160 L1440,320 L0,320 Z" />
</svg>
